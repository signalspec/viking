use core::str;
use std::iter;
pub struct Resources {
    resources: Vec<Resource>,
}

pub struct Resource {
    name: Box<str>,
    modes: Vec<Mode>,
}

pub struct Mode {
    name: Option<Box<str>>,
    protocol: u16,
    descriptor: Box<[u8]>,
}

impl Resources {
    pub fn parse(bytes: &[u8]) -> Result<Self, ()> {
        use viking_protocol::descriptor::*;
        let mut resources = vec![];

        for i in descriptors(bytes) {
            let (kind, body) = i?;

            match kind {
                DESCRIPTOR_TYPE_VIKING => {}
                DESCRIPTOR_TYPE_RESOURCE => {
                    resources.push(Resource {
                        name: "".into(),
                        modes: Vec::new(),
                    });
                }
                DESCRIPTOR_TYPE_MODE => {
                    let Some(resource) = resources.last_mut() else {
                        return Err(());
                    };
                    let protocol =
                        u16::from_le_bytes(body.get(0..2).ok_or(())?.try_into().unwrap());
                    let descriptor = body[2..].into();
                    resource.modes.push(Mode {
                        name: None,
                        protocol,
                        descriptor,
                    });

                    if resource.modes.len() > 254 {
                        return Err(());
                    }
                }
                DESCRIPTOR_TYPE_IDENTIFIER => {
                    let s = str::from_utf8(body).map_err(|_| ())?.into();
                    if let Some(resource) = resources.last_mut() {
                        if let Some(mode) = resource.modes.last_mut() {
                            mode.name = Some(s);
                        } else {
                            resource.name = s;
                        }
                    }
                }
                _ => {}
            }
        }

        if resources.len() > 63 {
            return Err(());
        }

        Ok(Resources { resources })
    }

    pub fn resources(&self) -> impl Iterator<Item = (u8, &Resource)> {
        (1..).zip(self.resources.iter())
    }

    pub fn find_resource(&self, name: &str) -> Option<u8> {
        Some((self.resources.iter().position(|r| *r.name == *name)? + 1) as u8)
    }

    pub fn resource(&self, id: u8) -> Option<&Resource> {
        self.resources.get(id as usize - 1)
    }
}

impl Resource {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn modes(&self) -> impl Iterator<Item = (u8, &Mode)> {
        (1..).zip(self.modes.iter())
    }

    pub fn find_mode(&self, protocol: u16) -> Option<u8> {
        self.modes
            .iter()
            .position(|r| r.protocol == protocol)
            .map(|i| (i + 1).try_into().unwrap())
    }

    pub fn find_mode_named(&self, name: &str) -> Option<u8> {
        self.modes
            .iter()
            .position(|r| r.name.as_deref().is_some_and(|n| n == name))
            .map(|i| (i + 1).try_into().unwrap())
    }

    pub fn mode(&self, id: u8) -> Option<&Mode> {
        self.modes.get(id as usize - 1)
    }
}

impl Mode {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn protocol(&self) -> u16 {
        self.protocol
    }

    pub fn descriptor(&self) -> &[u8] {
        &self.descriptor
    }
}

fn descriptors(mut bytes: &[u8]) -> impl Iterator<Item = Result<(u8, &[u8]), ()>> {
    iter::from_fn(move || {
        if bytes.len() == 0 {
            return None;
        }
        if bytes.len() < 2 {
            return Some(Err(()));
        }
        let len = bytes[0] as usize;
        let ty = bytes[1];

        let Some(body) = bytes.get(2..len) else {
            return Some(Err(()));
        };

        bytes = &bytes[len..];

        Some(Ok((ty, body)))
    })
}
