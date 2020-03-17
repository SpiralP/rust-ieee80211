use super::*;
use std::borrow::Cow;

pub struct ControlFrame<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> ControlFrame<'a> {
    pub fn new<T: Into<Cow<'a, [u8]>>>(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}

impl FrameTrait for ControlFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    fn addr1(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
    }

    fn destination_address(&self) -> Option<MacAddress> {
        None
    }
}
impl ControlFrameTrait for ControlFrame<'_> {}

pub trait ControlFrameTrait: FrameTrait {
    fn addr2(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
    }

    fn transmitter_address(&self) -> Option<MacAddress> {
        match self.subtype() {
            FrameSubtype::Control(subtype) => match subtype {
                ControlSubtype::BlockAckRequest
                | ControlSubtype::BlockAck
                | ControlSubtype::PSPoll
                | ControlSubtype::RTS => Some(self.addr2()),
                _ => None,
            },
            _ => unreachable!(),
        }
    }

    fn source_address(&self) -> Option<MacAddress> {
        None
    }

    fn bssid_address(&self) -> Option<MacAddress> {
        match self.subtype() {
            FrameSubtype::Control(subtype) => match subtype {
                ControlSubtype::PSPoll => Some(self.addr1()),
                ControlSubtype::CFEnd | ControlSubtype::CFEndCFAck => Some(self.addr2()),
                _ => None,
            },
            _ => unreachable!(),
        }
    }

    fn station_address(&self) -> Option<MacAddress> {
        None
    }
}
