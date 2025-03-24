use std::io::{self, Write};

use futures_lite::future::block_on;

fn main() {
    env_logger::init();
    let intf = block_on(viking_io::Interface::find(0x59e3, 0x2222)).unwrap();

    info(&intf).unwrap();
}

fn info(intf: &viking_io::Interface) -> Result<(), io::Error> {
    let mut w = io::stdout().lock();
    let desc = intf.descriptor();

    writeln!(w, "Version {}", desc.version())?;
    writeln!(w, "  Commands up to {} bytes", desc.max_cmd_len())?;
    writeln!(w, "  Response up to {} bytes", desc.max_res_len())?;

    for (_, resource) in desc.resources() {
        writeln!(w, "{}", resource.name())?;
        for (_, mode) in resource.modes() {
            writeln!(w, "  {} {:04X}", mode.name().unwrap_or(""), mode.protocol())?;
        }
    }
    Ok(())
}
