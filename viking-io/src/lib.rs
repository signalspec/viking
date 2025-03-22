use std::{
    mem::replace,
    sync::{
        Arc,
        atomic::{self, AtomicU64},
    },
    time::Duration,
};

use command::VarInt;
use descriptor::Resources;
use log::debug;
use nusb::{
    Endpoint,
    transfer::{Bulk, ControlIn, ControlOut, ControlType, In, Out, Recipient, TransferError},
};
use thiserror::Error;

pub mod command;
pub mod descriptor;

pub mod gpio;
pub mod i2c;
pub mod led;
pub mod spi;

use self::command::{Command, PayloadPattern, ResponsePattern, StaticResponsePattern};

#[derive(Debug)]
pub struct Error {
    msg: &'static str,
    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl Error {
    fn new(msg: &'static str, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            msg,
            source: Some(Box::new(source)),
        }
    }
}

impl From<&'static str> for Error {
    fn from(msg: &'static str) -> Self {
        Error { msg, source: None }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref()
    }
}

pub struct Interface {
    intf: nusb::Interface,
    cmd_eps: async_lock::Mutex<CmdShared>,
    ep_evt: Endpoint<Bulk, In>,
    resources_used: AtomicU64,
    descriptor: descriptor::Resources,
    max_command_len: usize,
    max_response_len: usize,
}

struct CmdShared {
    ep_req: Endpoint<Bulk, Out>,
    ep_res: Endpoint<Bulk, In>,
    seq: u8,
}

impl CmdShared {
    fn next_seq(&mut self) -> u8 {
        let seq = self.seq;
        self.seq = seq.wrapping_add(1);
        seq
    }
}

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("{0}")]
    Protocol(&'static str),

    #[error("{0}")]
    Usb(#[from] TransferError),
}

impl Interface {
    pub async fn find(vid: u16, pid: u16) -> Result<Arc<Self>, Error> {
        let dev = nusb::list_devices()
            .await
            .map_err(|e| Error::new("couldn't list devices", e))?
            .find(|d| d.vendor_id() == vid && d.product_id() == pid)
            .ok_or_else(|| Error::from("device not found"))?;

        let dev = dev
            .open()
            .await
            .map_err(|e| Error::new("couldn't open device", e))?;

        let desc = dev
            .active_configuration()
            .map_err(|e| Error::new("couldn't get active configuration", e))?;

        let intf_desc = desc
            .interface_alt_settings()
            .find(|intf| intf.class() == 0xff && intf.subclass() == 0x00 && intf.protocol() == 0x00)
            .ok_or_else(|| Error::from("no Viking interface found on device"))?;

        let intf_handle = dev
            .claim_interface(intf_desc.interface_number())
            .await
            .map_err(|e| Error::new("couldn't claim interface", e))?;

        Self::from_nusb(intf_handle).await
    }

    pub async fn from_nusb(intf: nusb::Interface) -> Result<Arc<Self>, Error> {
        intf.set_alt_setting(1)
            .await
            .map_err(|e| Error::new("failed to set interface alt setting", e))?;

        let desc = intf
            .descriptor()
            .ok_or(Error::from("interface descriptor not found"))?;

        let mut endpoints = desc.endpoints();
        let ep_req_addr = endpoints
            .next()
            .ok_or(Error::from("request endpoint not found"))?
            .address();
        let ep_res_addr = endpoints
            .next()
            .ok_or(Error::from("response endpoint not found"))?
            .address();
        let ep_evt_addr = endpoints
            .next()
            .ok_or(Error::from("event endpoint not found"))?
            .address();
        drop(endpoints);

        let ep_req = intf
            .endpoint::<Bulk, Out>(ep_req_addr)
            .map_err(|e| Error::new("failed to claim request endpoint", e))?;
        let ep_res = intf
            .endpoint::<Bulk, In>(ep_res_addr)
            .map_err(|e| Error::new("failed to claim response endpoint", e))?;
        let ep_evt = intf
            .endpoint::<Bulk, In>(ep_evt_addr)
            .map_err(|e| Error::new("failed to claim event endpoint", e))?;

        let descriptor = intf
            .control_in(
                ControlIn {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Interface,
                    request: viking_protocol::request::DESCRIBE_RESOURCES,
                    value: 0,
                    index: intf.interface_number() as u16,
                    length: 4096,
                },
                Duration::from_millis(100),
            )
            .await
            .map_err(|e| Error::new("failed to read Viking resource descriptors", e))?;

        let descriptor = descriptor::Resources::parse(&descriptor)
            .map_err(|_| Error::from("failed to parse Viking resource descriptors"))?;

        let this = Arc::new(Self {
            intf,
            cmd_eps: async_lock::Mutex::new(CmdShared {
                seq: 0,
                ep_req,
                ep_res,
            }),
            ep_evt,
            descriptor,
            resources_used: AtomicU64::new(0),
            max_command_len: 1023,
            max_response_len: 1023,
        });

        Ok(this)
    }

    pub fn resource(self: &Arc<Self>, name: &str) -> Result<Resource, ResourceError> {
        let id = self
            .descriptor()
            .find_resource(name)
            .ok_or(ResourceError::NotFound)?;
        let mask = 1 << id;
        if self
            .resources_used
            .fetch_or(mask, atomic::Ordering::Acquire)
            & mask
            != 0
        {
            return Err(ResourceError::Busy);
        }

        Ok(Resource {
            interface: self.clone(),
            id,
            mode_id: None,
        })
    }

    pub fn descriptor(&self) -> &Resources {
        &self.descriptor
    }

    async fn configure_resource(&self, resource: u8, mode: u8, data: &[u8]) -> Result<(), Error> {
        log::info!("configure resource {resource} as {mode}: {data:x?}");
        let intf_num = self.intf.interface_number();
        Ok(self
            .intf
            .control_out(
                ControlOut {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Interface,
                    request: viking_protocol::request::CONFIGURE_MODE,
                    value: (resource as u16) << 8 | mode as u16,
                    index: intf_num as u16,
                    data,
                },
                Duration::from_millis(100),
            )
            .await
            .map(drop)
            .map_err(|e| Error::new("configure mode failed", e))?)
    }

    pub fn batch(self: &Arc<Self>) -> CommandBatch {
        CommandBatch::new(self)
    }

    pub fn queue(self: &Arc<Self>) -> CommandQueue {
        CommandQueue::new(self)
    }

    pub async fn run<P: PayloadPattern, R: StaticResponsePattern>(
        self: &Arc<Self>,
        cmd: Command<P, R>,
    ) -> Result<R::StaticOutput, RequestError> {
        let mut batch = self.batch();
        let h = batch.push(cmd);
        let res = batch.run().await?;
        Ok(h.res.static_output(&res.res[2..]))
    }
}

fn cmd_delay(us: u32) -> Command<VarInt, ()> {
    Command::new(0, viking_protocol::protocol::cmd::DELAY, VarInt(us), ())
}

pub struct CommandBatch<'a> {
    intf: &'a Arc<Interface>,
    response_len: usize,
    req: Vec<u8>,
}

pub struct ResponseBatch {
    res: Vec<u8>,
}

pub struct ResponseHandle<R> {
    res: R,
    offset: usize,
}

impl<'a> CommandBatch<'a> {
    fn new(intf: &'a Arc<Interface>) -> Self {
        let req = vec![0, 0];

        CommandBatch {
            intf,
            response_len: 0,
            req,
        }
    }

    pub fn can_fit<P: PayloadPattern, R: ResponsePattern>(&mut self, cmd: &Command<P, R>) -> bool {
        self.req.len() + 1 + cmd.payload.len() <= self.intf.max_command_len
            && self.response_len + cmd.response.len() <= self.intf.max_response_len
    }

    pub fn push<P: PayloadPattern, R: ResponsePattern>(
        &mut self,
        cmd: Command<P, R>,
    ) -> ResponseHandle<R> {
        assert!(self.can_fit(&cmd));
        let offset = self.response_len;
        for b in cmd.bytes() {
            self.req.extend_from_slice(&[b]);
        }
        let res = cmd.response();
        let len = res.len();
        self.response_len += len;
        ResponseHandle { res, offset }
    }

    pub async fn run(mut self) -> Result<ResponseBatch, RequestError> {
        let mut lock = self.intf.cmd_eps.lock().await;
        let seq = lock.next_seq();
        self.req[0] = seq;

        let mut t_out = lock.ep_req.allocate(self.req.len());
        t_out.extend_from_slice(&self.req);
        let zlp = self.req.len() % lock.ep_req.max_packet_size() == 0;
        debug!("Send batch {:x?}", self.req);
        lock.ep_req.submit(t_out);

        if zlp {
            let t_zlp = lock.ep_req.allocate(0);
            lock.ep_req.submit(t_zlp);
        }

        lock.ep_req
            .next_complete()
            .await
            .status()
            .map_err(RequestError::Usb)?;
        if zlp {
            lock.ep_req
                .next_complete()
                .await
                .status()
                .map_err(RequestError::Usb)?;
        }

        let r = lock.ep_res.allocate(4096);
        lock.ep_res.submit(r);
        let res = lock.ep_res.next_complete().await;
        debug!("Response {res:x?}");
        res.status().map_err(RequestError::Usb)?;

        if res.len() < 2 {
            Err(RequestError::Protocol(
                "response packet too short for header",
            ))
        } else if res[0] != seq {
            Err(RequestError::Protocol("response sequence mismatch"))
        } else if res[1] != 0 {
            Err(RequestError::Protocol("device returned error status"))
        } else {
            Ok(ResponseBatch { res: res.to_vec() })
        }
    }
}

impl ResponseBatch {
    pub fn get<'a, R: ResponsePattern>(
        &'a self,
        h: ResponseHandle<R>,
    ) -> Result<R::Output<'a>, ()> {
        let slice = self.res.get(2 + h.offset..2 + h.offset + h.res.len());
        slice.map(|s| h.res.output(s)).ok_or(())
    }
}

pub struct CommandQueue<'a> {
    batch: CommandBatch<'a>,
    reads: Vec<(ResponseHandle<command::SliceResponse>, &'a mut [u8])>,
    error: Result<(), RequestError>,
}

impl<'a> CommandQueue<'a> {
    fn new(intf: &'a Arc<Interface>) -> Self {
        Self {
            batch: intf.batch(),
            reads: Vec::new(),
            error: Ok(()),
        }
    }

    async fn flush(&mut self) {
        let next_batch = self.batch.intf.batch();
        let batch = replace(&mut self.batch, next_batch);
        match batch.run().await {
            Ok(res) => {
                for (h, dest) in self.reads.drain(..) {
                    dest.copy_from_slice(res.get(h).unwrap());
                }
            }
            Err(err) => self.error = Err(err),
        }
    }

    pub async fn push<P: PayloadPattern, R: ResponsePattern>(&mut self, cmd: Command<P, R>) {
        if self.error.is_err() {
            return;
        }
        if !self.batch.can_fit(&cmd) {
            self.flush().await;
        }
        self.batch.push(cmd);
    }

    pub async fn push_read<P: PayloadPattern>(
        &mut self,
        cmd: Command<P, command::SliceResponse>,
        dest: &'a mut [u8],
    ) {
        if self.error.is_err() {
            return;
        }
        if !self.batch.can_fit(&cmd) {
            self.flush().await;
        }
        let h = self.batch.push(cmd);
        self.reads.push((h, dest));
    }

    pub async fn push_read_in_place(
        &mut self,
        buf: &'a mut [u8],
        cmd: impl FnOnce(&[u8]) -> Command<&[u8], command::SliceResponse>,
    ) {
        if self.error.is_err() {
            return;
        }
        let cmd = cmd(buf);
        if !self.batch.can_fit(&cmd) {
            self.flush().await;
        }
        let h = self.batch.push(cmd);
        self.reads.push((h, buf));
    }

    pub async fn finish(mut self) -> Result<(), RequestError> {
        self.flush().await;
        self.error
    }
}

pub struct Resource {
    interface: Arc<Interface>,
    id: u8,
    mode_id: Option<u8>,
}

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("not found")]
    NotFound,

    #[error("busy")]
    Busy,
}

impl Resource {
    #[inline]
    pub fn id(&self) -> u8 {
        self.id
    }

    #[inline]
    pub fn interface(&self) -> &Arc<Interface> {
        &self.interface
    }

    pub fn descriptor(&self) -> &descriptor::Resource {
        self.interface.descriptor.resource(self.id).unwrap()
    }

    pub async fn configure(&mut self, mode: u8, config: &[u8]) -> Result<(), Error> {
        self.interface
            .configure_resource(self.id, mode, config)
            .await?;
        self.mode_id = Some(mode);

        Ok(())
    }

    pub async fn deconfigure(&mut self) -> Result<(), Error> {
        self.interface.configure_resource(self.id, 0, &[]).await?;
        self.mode_id = Some(0);
        Ok(())
    }

    pub fn as_mode<M: ResourceMode>(self) -> Result<M::Builder, Error> {
        let mode = self
            .descriptor()
            .find_mode(M::PROTOCOL)
            .ok_or("no mode found by protocol")?;
        Ok(M::build(self, mode))
    }

    pub fn as_mode_named<M: ResourceMode>(self, name: &str) -> Result<M::Builder, Error> {
        let mode = self
            .descriptor()
            .find_mode_named(name)
            .ok_or("no mode found by name")?;

        if self.descriptor().mode(mode).unwrap().protocol() != M::PROTOCOL {
            Err("mode does not have specified protocol")?
        }

        Ok(M::build(self, mode))
    }
}

pub trait ResourceMode: Sized {
    const PROTOCOL: u16;
    type Builder;

    fn build(resource: Resource, mode: u8) -> Self::Builder;
}

macro_rules! resource_mode(
    ($mode:ident, $builder:ident, $protocol:expr) => {
        pub struct $builder {
            resource: crate::Resource,
            mode: u8,
        }

        impl crate::ResourceMode for $mode {
            const PROTOCOL: u16 = $protocol;
            type Builder = $builder;

            fn build(resource: Resource, mode: u8) -> Self::Builder {
                $builder { resource, mode }
            }
        }

        impl $builder {
            pub async fn enable(self) -> Result<$mode, crate::Error> {
                let mut resource = self.resource;
                resource.configure(self.mode, &[]).await?;
                Ok($mode { resource })
            }
        }
    }
);

pub(crate) use resource_mode;
