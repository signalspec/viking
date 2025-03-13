use std::io::{self, Write};

use futures_lite::future::block_on;

fn main() {
    env_logger::init();
    let intf = block_on(viking_io::Interface::find(0x59e3, 0x2222)).unwrap();

    info(&intf).unwrap();
}

fn info(intf: &viking_io::Interface) -> Result<(), io::Error> {
    let mut w = io::stdout().lock();
    for (_, resource) in intf.descriptor().resources() {
        writeln!(w, "{}", resource.name())?;
        for (_, mode) in resource.modes() {
            writeln!(w, "  {} {:04X}", mode.name().unwrap_or(""), mode.protocol())?;
        }
    }
    Ok(())
}
