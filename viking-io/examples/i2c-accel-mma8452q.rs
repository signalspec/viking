use std::error::Error;

use embedded_hal_async::i2c::I2c;
use tokio;
use viking_io::{Interface, i2c};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let dev = Interface::find(0x59e3, 0x2222).await?;

    let mut i2c = dev
        .resource("i2c")?
        .as_mode::<i2c::Controller>()?
        .enable()
        .await?;

    let mut who_am_i = [0u8];
    i2c.write_read(0x1D, &[0x0D], &mut who_am_i).await?;
    assert_eq!(who_am_i[0], 0x2A);

    i2c.write(0x1D, &[0x2A, 0x31]).await?; //CTRL_REG1 = Active, 6.25Hz

    loop {
        let mut data = [0; 7];
        i2c.write_read(0x1D, &[0x00], &mut data).await?;
        let status = data[0];
        let x = i16::from_be_bytes(data[1..3].try_into().unwrap());
        let y = i16::from_be_bytes(data[3..5].try_into().unwrap());
        let z = i16::from_be_bytes(data[5..7].try_into().unwrap());
        println!("status: {status:02X} x: {x:5} y: {y:5} z: {z:5}");
        tokio::time::sleep(std::time::Duration::from_millis(160)).await;
    }
}
