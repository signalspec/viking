use crate::{
    RequestError, Resource,
    command::{Command, ScalarResponse},
    resource_mode,
};
use viking_protocol::protocol::gpio::pin as protocol;

pub struct Gpio {
    pub(crate) resource: Resource,
}

resource_mode!(Gpio, GpioBuilder, protocol::PROTOCOL);

impl Gpio {
    pub fn id(&self) -> u8 {
        self.resource.id
    }

    pub fn cmd_float(&self) -> Command<(), ()> {
        Command::new(self.resource.id, protocol::cmd::FLOAT, (), ())
    }

    pub async fn float(&self) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_float()).await
    }

    pub fn cmd_read(&self) -> Command<(), ScalarResponse<u8>> {
        Command::new(
            self.resource.id,
            protocol::cmd::READ,
            (),
            ScalarResponse::new(),
        )
    }

    pub async fn read(&self) -> Result<u8, RequestError> {
        self.resource.interface.run(self.cmd_read()).await
    }

    pub fn cmd_write(&self, level: bool) -> Command<(), ()> {
        let cmd = if level {
            protocol::cmd::HIGH
        } else {
            protocol::cmd::LOW
        };
        Command::new(self.resource.id, cmd, (), ())
    }

    pub async fn write(&self, level: bool) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_write(level)).await
    }

    pub async fn high(&self) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_write(true)).await
    }

    pub async fn low(&self) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_write(false)).await
    }
}
