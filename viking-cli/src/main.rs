use std::{error::Error, io::{self, Write}};
use owo_colors::OwoColorize;

use futures_lite::future::block_on;
use viking_protocol::protocol;

fn main() {
    env_logger::init();
    info().unwrap();
}

fn info() -> Result<(), Box<dyn Error>> {
    let mut w = io::stdout().lock();

    for dev in block_on(viking_io::list_devices((), None))? {
        let product = dev.device.product_string().unwrap_or("unknown").trim_end_matches(" (Viking)");
        writeln!(w, "{} ({:04X}:{:04X}) {}",
            product.bold().green(),
            dev.device.vendor_id(),
            dev.device.product_id(),
            dev.device.serial_number().unwrap_or("").dimmed(),
        )?;
        match block_on(dev.open()) {
            Ok(intf) => {
                writeln!(w)?;
                interface_info(&mut w, &intf).unwrap();
            }
            Err(e) => {
                eprintln!("{} {e}", "Failed to open device:".red());
            }
        }
    }

    Ok(())
}

fn interface_info(w: &mut dyn Write, intf: &viking_io::Interface) -> Result<(), io::Error> {
    let desc = intf.descriptor();

    writeln!(w, "Protocol version {}", desc.version())?;
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
