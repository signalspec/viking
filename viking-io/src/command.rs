use std::{array, marker::PhantomData};

pub trait ResponsePattern: Clone {
    type Output<'a>;
    fn len(&self) -> usize;
    fn output<'a>(&self, buf: &'a [u8]) -> Self::Output<'a>;
}

pub trait StaticResponsePattern: ResponsePattern {
    type StaticOutput;
    fn static_output(&self, buf: &[u8]) -> Self::StaticOutput;
}

#[derive(Clone)]
pub struct ScalarResponse<T>(PhantomData<T>);

impl<T> ScalarResponse<T> {
    pub fn new() -> Self {
        ScalarResponse(PhantomData)
    }
}

impl ResponsePattern for ScalarResponse<u8> {
    type Output<'a> = u8;

    fn output(&self, buf: &[u8]) -> u8 {
        buf[0]
    }

    fn len(&self) -> usize {
        size_of::<u8>()
    }
}

impl StaticResponsePattern for ScalarResponse<u8> {
    type StaticOutput = u8;

    fn static_output(&self, buf: &[u8]) -> u8 {
        buf[0]
    }
}

impl ResponsePattern for () {
    type Output<'a> = ();

    fn output(&self, _buf: &[u8]) -> () {
        ()
    }

    fn len(&self) -> usize {
        size_of::<()>()
    }
}

impl StaticResponsePattern for () {
    type StaticOutput = ();

    fn static_output(&self, _buf: &[u8]) -> () {
        ()
    }
}

#[derive(Clone)]
pub struct SliceResponse(usize);

impl SliceResponse {
    pub(crate) fn new(len: usize) -> Self {
        Self(len)
    }
}

impl ResponsePattern for SliceResponse {
    type Output<'a> = &'a [u8];

    fn output<'a>(&self, buf: &'a [u8]) -> &'a [u8] {
        &buf[..self.0]
    }

    fn len(&self) -> usize {
        self.0
    }
}

pub trait PayloadPattern {
    fn len(&self) -> usize {
        self.bytes().count()
    }
    fn bytes(&self) -> impl Iterator<Item = u8>;
}

impl PayloadPattern for u8 {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        [*self].into_iter()
    }
}

impl PayloadPattern for () {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        [].into_iter()
    }
}

pub struct VarInt(pub u32);

impl PayloadPattern for VarInt {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        let mut bytes: [u8; 5] = array::from_fn(|i| ((self.0 >> i * 7) | 0x80) as u8);
        bytes[0] &= 0x7f;
        bytes.into_iter().rev().skip_while(|x| x == &0x80)
    }
}

#[test]
fn test_var_int() {
    assert_eq!(VarInt(0).bytes().collect::<Vec<u8>>(), vec![0]);
    assert_eq!(VarInt(127).bytes().collect::<Vec<u8>>(), vec![0x7F]);
    assert_eq!(VarInt(128).bytes().collect::<Vec<u8>>(), vec![0x81, 0x00]);
    assert_eq!(VarInt(16383).bytes().collect::<Vec<u8>>(), vec![0xFF, 0x7F]);
    assert_eq!(
        VarInt(16384).bytes().collect::<Vec<u8>>(),
        vec![0x81, 0x80, 0x00]
    );
    assert_eq!(
        VarInt(268435455).bytes().collect::<Vec<u8>>(),
        vec![0xff, 0xff, 0xff, 0x7f]
    );
    assert_eq!(
        VarInt(4294967295).bytes().collect::<Vec<u8>>(),
        vec![0x8f, 0xff, 0xff, 0xff, 0x7f]
    );
}

impl PayloadPattern for &[u8] {
    fn len(&self) -> usize {
        1 + <[u8]>::len(self).min(255)
    }
    fn bytes(&self) -> impl Iterator<Item = u8> {
        [<[u8]>::len(self)
            .try_into()
            .expect("slice must be less than 256 bytes")]
        .into_iter()
        .chain(self.iter().copied())
    }
}

pub struct Command<P, R> {
    pub(crate) resource: u8,
    pub(crate) cmd: u8,
    pub(crate) payload: P,
    pub(crate) response: R,
}

impl<P, R> Command<P, R>
where
    P: PayloadPattern,
    R: ResponsePattern,
{
    pub fn new(resource: u8, cmd: u8, payload: P, response: R) -> Self {
        Command {
            resource,
            cmd,
            payload,
            response,
        }
    }

    pub(crate) fn cmd_byte(&self) -> u8 {
        self.resource | self.cmd << 6
    }

    pub(crate) fn bytes(&self) -> impl Iterator<Item = u8> {
        [self.cmd_byte()].into_iter().chain(self.payload.bytes())
    }

    pub(crate) fn response(&self) -> R {
        self.response.clone()
    }
}
