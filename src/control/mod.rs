use super::*;

pub struct ControlFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> ControlFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}

impl<'a> FrameTrait<'a> for ControlFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
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

  fn destination_address(&self) -> Option<MacAddress> {
    None
  }

  fn source_address(&self) -> Option<MacAddress> {
    None
  }
}
impl<'a> ControlFrameTrait<'a> for ControlFrame<'a> {}

pub trait ControlFrameTrait<'a>: FrameTrait<'a> {
  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
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
