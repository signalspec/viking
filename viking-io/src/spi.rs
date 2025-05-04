use std::{sync::Arc, u16};

use crate::{
    RequestError, Resource, cmd_delay,
    command::{Command, SliceResponse},
    gpio::Gpio,
    resource_mode,
};
use viking_protocol::protocol::spi::{controller, sck_pin, sdi_pin, sdo_pin};

pub struct Controller {
    resource: Resource,
}

resource_mode!(Controller, ControllerBuilder, controller::PROTOCOL);

impl Controller {
    pub fn cmd_transfer<'a>(&self, tx: &'a [u8]) -> Command<&'a [u8], SliceResponse> {
        Command::new(
            self.resource.id,
            controller::cmd::TRANSFER,
            tx,
            SliceResponse::new(tx.len()),
        )
    }
}

#[derive(Debug)]
pub struct Error;

impl embedded_hal_async::spi::Error for Error {
    fn kind(&self) -> embedded_hal_async::spi::ErrorKind {
        todo!()
    }
}

impl From<RequestError> for Error {
    fn from(value: RequestError) -> Self {
        Error
    }
}

impl embedded_hal_async::spi::ErrorType for Controller {
    type Error = Error;
}

impl embedded_hal_async::spi::SpiBus for Controller {
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let mut queue = self.resource.interface.queue();
        let zeros = [0; 255];
        for dest in words.chunks_mut(255) {
            queue
                .push_read(self.cmd_transfer(&zeros[..dest.len()]), dest)
                .await;
        }
        Ok(queue.finish().await?)
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let mut queue = self.resource.interface.queue();
        for src in words.chunks(255) {
            queue.push(self.cmd_transfer(src)).await;
        }
        Ok(queue.finish().await?)
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        assert_eq!(read.len(), write.len());
        let mut queue = self.resource.interface.queue();
        for (dest, src) in read.chunks_mut(255).zip(write.chunks(255)) {
            queue.push_read(self.cmd_transfer(src), dest).await;
        }
        Ok(queue.finish().await?)
    }

    async fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let mut queue = self.resource.interface.queue();
        for chunk in words.chunks_mut(255) {
            queue
                .push_read_in_place(chunk, |b| self.cmd_transfer(b))
                .await;
        }
        Ok(queue.finish().await?)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub struct SckPin {
    #[allow(unused)]
    resource: Resource,
}

resource_mode!(SckPin, SckPinBuilder, sck_pin::PROTOCOL);

pub struct SdoPin {
    #[allow(unused)]
    resource: Resource,
}

resource_mode!(SdoPin, SdoPinBuilder, sdo_pin::PROTOCOL);

pub struct SdiPin {
    #[allow(unused)]
    resource: Resource,
}

resource_mode!(SdiPin, SdiPinBuilder, sdi_pin::PROTOCOL);

pub struct Device<C = Arc<Controller>> {
    controller: C,
    chip_select: Gpio,
}

impl<C: AsRef<Controller>> Device<C> {
    pub fn new(controller: C, chip_select: Gpio) -> Self {
        assert!(Arc::ptr_eq(
            &controller.as_ref().resource.interface,
            &chip_select.resource.interface
        ));
        Self {
            controller,
            chip_select,
        }
    }
}

impl<C: AsRef<Controller>> embedded_hal_async::spi::ErrorType for Device<C> {
    type Error = Error;
}

impl<C: AsRef<Controller>> embedded_hal_async::spi::SpiDevice for Device<C> {
    async fn transaction(
        &mut self,
        operations: &mut [embedded_hal_async::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        let controller = self.controller.as_ref();
        let mut queue = controller.resource.interface.queue();

        let zeros = [0; 255];

        queue.push(self.chip_select.cmd_write(false)).await;

        for op in operations.into_iter() {
            match op {
                embedded_hal_async::spi::Operation::DelayNs(ns) => {
                    queue
                        .push(cmd_delay(ns.div_ceil(1000).try_into().unwrap_or(u16::MAX)))
                        .await;
                }

                embedded_hal_async::spi::Operation::Read(buf) => {
                    for chunk in buf.chunks_mut(255) {
                        queue
                            .push_read(controller.cmd_transfer(&zeros[..chunk.len()]), chunk)
                            .await;
                    }
                }

                embedded_hal_async::spi::Operation::Write(buf) => {
                    for chunk in buf.chunks(255) {
                        queue.push(controller.cmd_transfer(chunk)).await;
                    }
                }

                embedded_hal_async::spi::Operation::Transfer(rx, tx) => {
                    assert_eq!(rx.len(), tx.len());
                    for (rx_chunk, tx_chunk) in rx.chunks_mut(255).zip(tx.chunks(255)) {
                        queue
                            .push_read(controller.cmd_transfer(tx_chunk), rx_chunk)
                            .await;
                    }
                }

                embedded_hal_async::spi::Operation::TransferInPlace(buf) => {
                    for chunk in buf.chunks_mut(255) {
                        queue
                            .push_read_in_place(chunk, |b| controller.cmd_transfer(b))
                            .await;
                    }
                }
            }
        }
        queue.push(self.chip_select.cmd_write(true)).await;
        queue.finish().await?;

        Ok(())
    }
}
