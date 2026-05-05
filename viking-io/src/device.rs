use std::sync::Arc;
use crate::{Interface, Error};

pub trait DeviceMatcher {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8>;
}

impl DeviceMatcher for (u16, u16) {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8> {
        if dev.vendor_id() == self.0 && dev.product_id() == self.1 {
            dev.interfaces()
                .position(|intf| intf.class() == 0xff)
                .map(|i| i as u8)
        } else {
            None
        }
    }
}

impl DeviceMatcher for (u16, u16, u8) {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8> {
        if dev.vendor_id() == self.0 && dev.product_id() == self.1 {
            Some(self.2)
        } else {
            None
        }
    }
}

impl<T: DeviceMatcher> DeviceMatcher for &[T] {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8> {
        self.iter().find_map(|m| m.test(dev))
    }
}

impl<T: DeviceMatcher> DeviceMatcher for Option<T> {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8> {
        if let Some(m) = self {
            m.test(dev)
        } else {
            ().test(dev)
        }
    }
}

impl DeviceMatcher for () {
    fn test(&self, dev: &nusb::DeviceInfo) -> Option<u8> {
        (0x59e3, 0x2222).test(dev)
    }
}

pub struct FoundDevice {
    pub device: nusb::DeviceInfo,
    pub intf: u8,
}

impl FoundDevice {
    pub async fn open(&self) -> Result<Arc<Interface>, Error> {
        let dev = self.device
            .open()
            .await
            .map_err(|e| Error::new("couldn't open device", e))?;

        let intf_handle = dev
            .claim_interface(self.intf)
            .await
            .map_err(|e| Error::new("couldn't claim interface", e))?;

        Interface::from_nusb(intf_handle).await
    }
}

pub async fn list_devices(matcher: impl DeviceMatcher, serial: Option<&str>) -> Result<Vec<FoundDevice>, Error> {
    Ok(nusb::list_devices()
        .await
        .map_err(|e| Error::new("couldn't list devices", e))?
        .filter(|d| serial.is_none() || d.serial_number() == serial)
        .filter_map(|device| matcher.test(&device).map(|intf| FoundDevice { device, intf }))
        .collect::<Vec<_>>())
}
