use crate::{RequestError, Resource, command::Command, resource_mode};
use viking_protocol::protocol::led::binary as protocol;

pub struct Led {
    pub(crate) resource: Resource,
}

resource_mode!(Led, GpioBuilder, protocol::PROTOCOL);

impl Led {
    pub fn id(&self) -> u8 {
        self.resource.id
    }

    pub fn cmd_set(&self, level: bool) -> Command<(), ()> {
        let cmd = if level {
            protocol::cmd::ON
        } else {
            protocol::cmd::OFF
        };
        Command::new(self.resource.id, cmd, (), ())
    }

    pub async fn set(&self, level: bool) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_set(level)).await
    }

    pub async fn on(&self) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_set(true)).await
    }

    pub async fn off(&self) -> Result<(), RequestError> {
        self.resource.interface.run(self.cmd_set(false)).await
    }
}
