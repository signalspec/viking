use std::error::Error;

use viking_io::{Interface, led::Led};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dev = Interface::find(0x59e3, 0x2222).await?;

    let led = dev.resource("led")?.as_mode::<Led>()?.enable().await?;

    loop {
        led.off().await?;
        tokio::time::sleep(std::time::Duration::from_millis(800)).await;
        led.on().await?;
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}
