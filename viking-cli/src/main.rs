use std::{thread, time::Duration};

use futures_lite::future::block_on;
fn main() {
    let Some(intf) = viking_io::Interface::find(0x59e3, 0x2222) else {
        eprintln!("Device not found");
        return;
    };

    block_on(info(&intf)).unwrap();
    block_on(intf.configure(3, 1, &[])).unwrap();

    loop {
        println!("{:?}", block_on(intf.run(vec![0x3 | 2 << 6])));
        thread::sleep(Duration::from_millis(100));
        println!("{:?}", block_on(intf.run(vec![0x3 | 3 << 6])));
        thread::sleep(Duration::from_millis(100));
    }
}

async fn info(intf: &viking_io::Interface) -> Result<(), viking_io::Error> {
    for (resource_id, resource_name) in (1..).zip(intf.list_resouces().await?) {
        if resource_name.is_empty() {
            continue;
        }
        println!("{resource_name}");
        for mode_name in intf.list_modes(resource_id).await? {
            if mode_name.is_empty() {
                continue;
            }
            println!("  {mode_name}");
        }
    }
    Ok(())
}
