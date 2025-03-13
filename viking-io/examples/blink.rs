use std::error::Error;

use futures_lite::future::block_on;
use viking_io::{Interface, gpio::Gpio};

fn main() {
    env_logger::init();
    block_on(inner()).unwrap()
}

async fn inner() -> Result<(), Box<dyn Error>> {
    let dev = Interface::find(0x59e3, 0x2222).await?;

    let led = dev.resource("pb30")?.as_mode::<Gpio>()?.enable().await?;

    loop {
        led.write(false).await?;
        led.write(true).await?;
    }
}
