use super::*;

pub struct ControlFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> ControlFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
  }
  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
  }
}
impl<'a> FrameTrait<'a> for ControlFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> IEEE802_11FrameTrait<'a> for ControlFrame<'a> {
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

  fn bssid_address(&self) -> Option<MacAddress> {
    match self.subtype() {
      FrameSubtype::Control(subtype) => match subtype {
        ControlSubtype::PSPoll => Some(self.addr1()),
        ControlSubtype::CFEnd | ControlSubtype::CFEndCFACK => Some(self.addr2()),
        _ => None,
      },
      _ => unreachable!(),
    }
  }
}
impl<'a> ControlFrameTrait<'a> for ControlFrame<'a> {}

pub trait ControlFrameTrait<'a>: IEEE802_11FrameTrait<'a> {
  // fns unique to Control Frames
}

/*

impl<'a> IEEE802_11FrameTrait<'a> for ManagementFrame<'a> {
  /// Destination Address
  /// Who the packet is destined for.
  fn destination_address(&self) -> Option<MacAddress> {
    match self.type_() {
      FrameType::Control => None,
      FrameType::Data => match self.ds_status() {
        DSStatus::FromSTAToDS => Some(self.addr3()),
        DSStatus::WDSOrMesh => Some(self.addr3()),
        _ => None,
      },
      // fall back to receiver
      _ => Some(self.receiver_address()),
    }
  }
}
impl<'a> ManagementFrameTrait<'a> for ManagementFrame<'a> {}

pub trait ManagementFrameTrait<'a>: IEEE802_11FrameTrait<'a> {
  /// Transmitter Address
  /// Who this packet came from wirelessly.
  fn transmitter_address(&self) -> Option<MacAddress> {
    match self.subtype() {
      FrameSubtype::Management(_) | FrameSubtype::Data(_) => Some(self.addr2()),
      FrameSubtype::Control(subtype) => match subtype {
        ControlSubtype::BlockAckRequest
        | ControlSubtype::BlockAck
        | ControlSubtype::PSPoll
        | ControlSubtype::RTS => Some(self.addr2()),
        _ => None,
      },
      _ => None,
    }
  }

  /// Source Address
  /// Who the packet came from.
  fn source_address(&self) -> Option<MacAddress> {
    match self.type_() {
      FrameType::Control => None,
      FrameType::Data => match self.ds_status() {
        DSStatus::FromDSToSTA => Some(self.addr3()),
        DSStatus::FromSTAToDS => Some(self.addr2()),
        DSStatus::WDSOrMesh => Some(self.addr4()),
        _ => None,
      },
      // fall back to transmitter?
      _ => self.transmitter_address(),
    }
  }

  /// Basic Service Set Address (BSSID)
  /// Basic Service Set ID for Multicast.
  fn bssid_address(&self) -> Option<MacAddress> {
    match self.subtype() {
      FrameSubtype::Management(_) => Some(self.addr3()),
      FrameSubtype::Data(_) => match self.ds_status() {
        DSStatus::FromDSToSTA => Some(self.addr2()),
        DSStatus::FromSTAToDS => Some(self.addr1()),
        DSStatus::NotLeavingDSOrADHOC => Some(self.addr3()),
        _ => None,
      },
      FrameSubtype::Control(subtype) => match subtype {
        ControlSubtype::PSPoll => Some(self.addr1()),
        ControlSubtype::CFEnd | ControlSubtype::CFEndCFACK => Some(self.addr2()),
        _ => None,
      },
      _ => None,
    }
  }

  /// Station Address
  fn station_address(&self) -> Option<MacAddress> {
    match self.type_() {
      FrameType::Data => match self.ds_status() {
        DSStatus::FromDSToSTA => Some(self.addr1()),
        DSStatus::FromSTAToDS => Some(self.addr2()),
        _ => None,
      },
      _ => None,
    }
  }

  fn frag_seq(&self) -> Option<u16> {
    match self.type_() {
      FrameType::Management | FrameType::Data => {
        let n = u16::from(self.bytes()[22]) | ((u16::from(self.bytes()[23])) << 8);
        Some(n)
      }
      _ => None,
    }
  }

  /// Fragment Number
  fn fragment_number(&self) -> Option<u8> {
    self
      .frag_seq()
      .map(|frag_seq| (frag_seq & 0b0000_1111) as u8)
  }

  /// Sequence Number
  fn sequence_number(&self) -> Option<u16> {
    self.frag_seq().map(|frag_seq| frag_seq >> 4)
  }
}

*/
