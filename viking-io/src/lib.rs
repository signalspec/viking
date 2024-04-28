use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient, RequestBuffer};

pub type Error = Box<dyn std::error::Error>;

pub struct Interface {
    intf: nusb::Interface,
    intf_num: u8,
    ep_req: u8,
    ep_res: u8,
    ep_evt: u8,
}

impl Interface {
    pub fn find(vid: u16, pid: u16) -> Option<Self> {
        let dev = nusb::list_devices()
            .ok()?
            .find(|d| d.vendor_id() == vid && d.product_id() == pid)?;
        let dev = dev.open().ok()?;

        let desc = dev.active_configuration().ok()?;

        let intf_desc = desc.interface_alt_settings().find(|intf| {
            intf.class() == 0xff && intf.subclass() == 0x00 && intf.protocol() == 0x00
        })?;

        let intf_handle = dev.claim_interface(intf_desc.interface_number()).ok()?;

        Self::from_nusb(intf_desc, intf_handle)
    }

    pub fn from_nusb(
        desc: nusb::descriptors::InterfaceAltSetting,
        intf: nusb::Interface,
    ) -> Option<Self> {
        let intf_num = desc.interface_number();
        let [ep_req, ep_res, ep_evt] = desc
            .endpoints()
            .map(|e| e.address())
            .collect::<Vec<_>>()
            .try_into()
            .ok()?;

        Some(Self {
            intf,
            intf_num,
            ep_req,
            ep_res,
            ep_evt,
        })
    }

    pub async fn list_resouces(&self) -> Result<Vec<String>, Error> {
        let data = self
            .intf
            .control_in(ControlIn {
                control_type: ControlType::Vendor,
                recipient: Recipient::Interface,
                request: viking_protocol::request::LIST_RESOURCES,
                value: 0,
                index: self.intf_num as u16,
                length: 4096,
            })
            .await
            .into_result()?;

        Ok(data
            .split(|&b| b == 0)
            .map(|n| String::from_utf8(n.into()))
            .collect::<Result<Vec<String>, _>>()?)
    }

    pub async fn list_modes(&self, resource: u8) -> Result<Vec<String>, Error> {
        let data = self
            .intf
            .control_in(ControlIn {
                control_type: ControlType::Vendor,
                recipient: Recipient::Interface,
                request: viking_protocol::request::LIST_MODES,
                value: (resource as u16) << 8,
                index: self.intf_num as u16,
                length: 4096,
            })
            .await
            .into_result()?;

        Ok(data
            .split(|&b| b == 0)
            .map(|n| String::from_utf8(n.into()))
            .collect::<Result<Vec<String>, _>>()?)
    }

    pub async fn describe_mode(&self, resource: u8, mode: u8) -> Result<Vec<u8>, Error> {
        Ok(self
            .intf
            .control_in(ControlIn {
                control_type: ControlType::Vendor,
                recipient: Recipient::Interface,
                request: viking_protocol::request::LIST_MODES,
                value: (resource as u16) << 8 | mode as u16,
                index: self.intf_num as u16,
                length: 4096,
            })
            .await
            .into_result()?)
    }

    pub async fn configure(&self, resource: u8, mode: u8, data: &[u8]) -> Result<(), Error> {
        Ok(self
            .intf
            .control_out(ControlOut {
                control_type: ControlType::Vendor,
                recipient: Recipient::Interface,
                request: viking_protocol::request::CONFIGURE_MODE,
                value: (resource as u16) << 8 | mode as u16,
                index: self.intf_num as u16,
                data,
            })
            .await
            .into_result()
            .map(drop)?)
    }

    pub async fn run(&self, data: Vec<u8>) -> Result<Vec<u8>, Error> {
        self.intf.bulk_out(self.ep_req, data).await.into_result()?;
        Ok(self
            .intf
            .bulk_in(self.ep_res, RequestBuffer::new(4096))
            .await
            .into_result()?)
    }
}
