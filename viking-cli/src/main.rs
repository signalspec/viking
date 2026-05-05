use std::io::{self, Write};
use owo_colors::OwoColorize;

use futures_lite::future::block_on;
use viking_protocol::protocol;

fn main() {
    env_logger::init();
    let intf = block_on(viking_io::Interface::find(0x59e3, 0x2222)).unwrap();

    info(&intf).unwrap();
}

fn info(intf: &viking_io::Interface) -> Result<(), io::Error> {
    let mut w = io::stdout().lock();
    let desc = intf.descriptor();

    writeln!(w, "Viking protocol version {}", desc.version())?;
    writeln!(w, "Commands up to {} bytes", desc.max_cmd_len())?;
    writeln!(w, "Response up to {} bytes", desc.max_res_len())?;
    writeln!(w)?;

    for (_, resource) in desc.resources() {
        writeln!(w, "{}", resource.name().bold().blue())?;
        for (_, mode) in resource.modes() {
            let protocol_id = mode.protocol();
            if let Some(protocol_name) = protocol::protocol_name(protocol_id) {
                write!(w, "    {protocol_name}")?;
            } else {
                write!(w, "    unknown({protocol_id:04X})")?;
            };

            if let Some(mode_name) = mode.name() {
                write!(w, " \"{mode_name}\"")?;
            }

            writeln!(w)?;
        }
    }
    Ok(())
}
