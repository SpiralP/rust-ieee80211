use super::*;
use bytes::Bytes;

pub struct ControlFrame {
    bytes: Bytes,
}

impl ControlFrame {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }
}

impl FrameTrait for ControlFrame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    fn addr1(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
    }

    fn destination_address(&self) -> Option<MacAddress> {
        None
    }
}
impl ControlFrameTrait for ControlFrame {}

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
