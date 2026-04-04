use crate::{
    RequestError, Resource,
    command::{Command, ScalarResponse, SliceResponse},
    resource_mode,
};
use embedded_hal_async::i2c::Operation;
use nusb::transfer::TransferError;
use thiserror::Error;
use viking_protocol::protocol::i2c::{controller, scl, sda};

pub struct Controller {
    resource: Resource,
}

resource_mode!(Controller, ControllerBuilder, controller::PROTOCOL);

impl Controller {
    pub fn cmd_start(&self, addr: u8) -> Command<u8, ()> {
        Command::new(
            self.resource.id,
            controller::cmd::START,
            addr,
            (),
        )
    }

    pub fn cmd_read(&self, len: u8) -> Command<u8, SliceResponse> {
        Command::new(
            self.resource.id,
            controller::cmd::READ,
            len,
            SliceResponse::new(len as usize),
        )
    }

    pub fn cmd_write<'a>(&self, data: &'a [u8]) -> Command<&'a [u8], ()> {
        Command::new(self.resource.id, controller::cmd::WRITE, data, ())
    }

    pub fn cmd_stop(&self) -> Command<(), ()> {
        Command::new(self.resource.id, controller::cmd::STOP, (), ())
    }
}

#[derive(Debug, Error)]
#[error("i2c error")]
pub enum Error {
    #[error("skipped due to prior error")]
    PriorError,

    #[error("unexpected response status {0:02X}")]
    Status(u8),

    #[error("{0}")]
    Protocol(&'static str),

    #[error("{0}")]
    Usb(#[from] TransferError),

    #[error("address not acknowledged")]
    AddrNack,

    #[error("data not acknowledged")]
    DataNack,

    #[error("arbitration lost")]
    ArbitrationLoss,

    #[error("timeout")]
    Timeout,

    #[error("unsuported command sequence")]
    Unsupported
}

impl embedded_hal_async::i2c::Error for Error {
    fn kind(&self) -> embedded_hal_async::i2c::ErrorKind {
        use embedded_hal_async::i2c::{ErrorKind, NoAcknowledgeSource};

        match self {
            Error::AddrNack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address),
            Error::DataNack => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data),
            Error::ArbitrationLoss => ErrorKind::ArbitrationLoss,
            _ => ErrorKind::Other,
        }
    }
}

impl From<RequestError> for Error {
    fn from(v: RequestError) -> Self {
        use viking_protocol::errors;
        match v {
            RequestError::PriorError => Self::PriorError,
            RequestError::Protocol(msg) => Self::Protocol(msg),
            RequestError::Usb(e) => Self::Usb(e),
            RequestError::Status(errors::ERR_ADDR_NACK) => Self::AddrNack,
            RequestError::Status(errors::ERR_DATA_NACK) => Self::DataNack,
            RequestError::Status(errors::ERR_ARBITRATION_LOST) => Self::ArbitrationLoss,
            RequestError::Status(errors::ERR_TIMEOUT) => Self::Timeout,
            RequestError::Status(errors::ERR_INVALID_STATE | errors::ERR_INVALID_ARG) => Self::Unsupported,
            RequestError::Status(status) => Self::Status(status),
        }
    }
}

impl embedded_hal_async::i2c::ErrorType for Controller {
    type Error = Error;
}

impl embedded_hal_async::i2c::I2c for Controller {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut queue = self.resource.interface.queue();

        let mut last_dir = None;

        for op in operations {
            match op {
                Operation::Read(r) => {
                    if last_dir != Some(true) {
                        queue.push(self.cmd_start(address << 1 | 1)).await;
                        last_dir = Some(true);
                    }
                    for chunk in r.chunks_mut(255) {
                        queue
                            .push_read(self.cmd_read(chunk.len() as u8), chunk)
                            .await;
                    }
                }
                Operation::Write(w) => {
                    if last_dir != Some(false) {
                        queue.push(self.cmd_start(address << 1)).await;
                        last_dir = Some(false);
                    }
                    for chunk in w.chunks(255) {
                        queue.push(self.cmd_write(chunk)).await;
                    }
                }
            }
        }

        if last_dir.is_some() {
            queue.push(self.cmd_stop()).await;
            queue.finish().await?;
        }
        Ok(())
    }
}

pub struct Sda {
    #[allow(unused)]
    resource: Resource,
}

resource_mode!(Sda, SdaBuilder, sda::PROTOCOL);

pub struct Scl {
    #[allow(unused)]
    resource: Resource,
}

resource_mode!(Scl, SclBuilder, scl::PROTOCOL);
