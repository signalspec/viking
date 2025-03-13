use std::{error::Error, sync::Arc};

use futures_lite::future::block_on;
use viking_io::{gpio::Gpio, spi};

use embedded_hal_async::spi::{Operation, SpiDevice};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let intf = block_on(viking_io::Interface::find(0x59e3, 0x2222))?;

    let spi = Arc::new(
        intf.resource("spi")?
            .as_mode::<spi::Controller>()?
            .enable()
            .await?,
    );
    let cs = intf.resource("cs")?.as_mode::<Gpio>()?.enable().await?;
    let ce = intf.resource("ce")?.as_mode::<Gpio>()?.enable().await?;

    const CONFIG: u8 = 0x00;
    const EN_AA: u8 = 0x01;
    const EN_RXADDR: u8 = 0x02;
    const SETUP_AW: u8 = 0x03;
    const SETUP_RETR: u8 = 0x04;
    const RF_CH: u8 = 0x05;
    const RF_SETUP: u8 = 0x06;
    const STATUS: u8 = 0x07;
    const RX_ADDR_P0: u8 = 0x0A;
    const RX_PW_P0: u8 = 0x11;
    const TX_ADDR: u8 = 0x10;

    const CONFIG_PRIM_RX: u8 = 1 << 0;
    const CONFIG_PWR_UP: u8 = 1 << 1;
    const CONFIG_CRCO: u8 = 1 << 2;
    const CONFIG_EN_CRC: u8 = 1 << 3;

    const STATUS_RX_DR: u8 = 1 << 6;
    const STATUS_TX_DS: u8 = 1 << 5;

    const RF_SETUP_DR_LOW: u8 = 1 << 5;
    const RF_SETUP_PWR_0DBM: u8 = 0b11 << 1;

    let mut nrf = NrfSpi {
        spi: spi::Device::new(spi, cs),
    };

    let config = CONFIG_EN_CRC | CONFIG_CRCO;
    nrf.write_reg(CONFIG, config);

    let read_config = nrf.read_reg(CONFIG);
    if config != read_config {
        eprintln!("set config to {config:02x}, read back {read_config:02x}");
        std::process::exit(3);
    }

    let channel = 23;
    let pkt_len = 8;

    nrf.write_reg(EN_AA, 0);
    nrf.write_reg(SETUP_RETR, 0);
    nrf.write_reg(SETUP_AW, 0b10);

    nrf.write_reg_bytes(TX_ADDR, &[0x23, 0x45, 0x23, 0xC1]);
    nrf.write_reg(RX_PW_P0 as u8, pkt_len);

    nrf.write_reg(RF_CH, channel);
    nrf.write_reg(RF_SETUP, RF_SETUP_DR_LOW | RF_SETUP_PWR_0DBM);

    let config = config | CONFIG_PWR_UP;
    nrf.write_reg(CONFIG, config);

    let read_config = nrf.read_reg(CONFIG);
    if config != read_config {
        eprintln!("set config to {config:02x}, read back {read_config:02x}");
        std::process::exit(3);
    }

    for i in 0u64.. {
        println!("sending: {i}");
        nrf.write_pkt(&i.to_le_bytes());

        block_on(ce.write(true))?;
        block_on(ce.write(false)).unwrap();

        println!("status: {:08b}", nrf.read_reg(STATUS));

        nrf.write_reg(STATUS, STATUS_TX_DS);

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

struct NrfSpi {
    spi: spi::Device,
}

impl NrfSpi {
    fn read_reg(&mut self, reg: u8) -> u8 {
        let tx = [reg & 0x1f, 0];
        let mut rx = [0, 0];
        block_on(
            self.spi
                .transaction(&mut [Operation::Transfer(&mut rx, &tx)]),
        )
        .unwrap();
        rx[1]
    }

    fn write_reg(&mut self, reg: u8, val: u8) {
        let tx = [0b0010_0000 | reg & 0x1f, val];
        eprintln!("write_reg: {reg:02x} {val:02x}");
        block_on(self.spi.transaction(&mut [Operation::Write(&tx)])).unwrap();
    }

    fn write_reg_bytes(&mut self, reg: u8, val: &[u8]) {
        let tx = [0b0010_0000 | reg & 0x1f];
        block_on(
            self.spi
                .transaction(&mut [Operation::Write(&tx), Operation::Write(val)]),
        )
        .unwrap();
    }

    fn read_pkt(&mut self) -> (u8, [u8; 32]) {
        let tx = [0b0110_0001];
        let mut status = [0];
        let mut data = [0; 32];

        block_on(self.spi.transaction(&mut [
            Operation::Transfer(&mut status, &tx),
            Operation::Read(&mut data),
        ]))
        .unwrap();

        (status[0], data)
    }

    fn write_pkt(&mut self, data: &[u8]) {
        let cmd = [0b1010_0000];

        block_on(
            self.spi
                .transaction(&mut [Operation::Write(&cmd), Operation::Write(&data)]),
        )
        .unwrap();
    }
}
