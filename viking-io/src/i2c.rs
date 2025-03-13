use crate::{
    RequestError, Resource,
    command::{Command, ScalarResponse, SliceResponse},
    resource_mode,
};
use embedded_hal_async::i2c::Operation;
use thiserror::Error;
use viking_protocol::protocol::i2c::{controller, scl, sda};

pub struct Controller {
    resource: Resource,
}

resource_mode!(Controller, ControllerBuilder, controller::PROTOCOL);

impl Controller {
    pub fn cmd_start(&self, addr: u8) -> Command<u8, ScalarResponse<u8>> {
        Command::new(
            self.resource.id,
            controller::cmd::START,
            addr,
            ScalarResponse::new(),
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
pub struct Error;

impl embedded_hal_async::i2c::Error for Error {
    fn kind(&self) -> embedded_hal_async::i2c::ErrorKind {
        todo!()
    }
}

impl From<RequestError> for Error {
    fn from(_value: RequestError) -> Self {
        Error
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
