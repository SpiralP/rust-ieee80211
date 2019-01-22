use super::*;

pub struct DataFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> DataFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
  }
  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
  }
  fn addr3(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[16..22]).unwrap()
  }
  fn addr4(&self) -> MacAddress {
    // only on Data Mesh types
    // after frag/seq numbers
    MacAddress::from_bytes(&self.bytes()[24..30]).unwrap()
  }
}
impl<'a> FrameTrait<'a> for DataFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> IEEE802_11FrameTrait<'a> for DataFrame<'a> {
  fn transmitter_address(&self) -> Option<MacAddress> {
    Some(self.addr2())
  }

  fn destination_address(&self) -> Option<MacAddress> {
    match self.ds_status() {
      DSStatus::FromDSToSTA => Some(self.addr1()),
      DSStatus::FromSTAToDS => Some(self.addr3()),
      DSStatus::WDSOrMesh => Some(self.addr3()),
      // fall back to receiver
      _ => Some(self.receiver_address()),
    }
  }

  /// Source Address
  /// Who the packet came from.
  fn source_address(&self) -> Option<MacAddress> {
    match self.ds_status() {
      DSStatus::FromDSToSTA => Some(self.addr3()),
      DSStatus::FromSTAToDS => Some(self.addr2()),
      DSStatus::WDSOrMesh => Some(self.addr4()),
      // fall back to transmitter
      _ => self.transmitter_address(),
    }
  }

  fn bssid_address(&self) -> Option<MacAddress> {
    match self.ds_status() {
      DSStatus::FromDSToSTA => Some(self.addr2()),
      DSStatus::FromSTAToDS => Some(self.addr1()),
      DSStatus::NotLeavingDSOrADHOC => Some(self.addr3()),
      _ => None,
    }
  }

  fn station_address(&self) -> Option<MacAddress> {
    match self.ds_status() {
      DSStatus::FromDSToSTA => Some(self.addr1()),
      DSStatus::FromSTAToDS => Some(self.addr2()),
      _ => None,
    }
  }
}
impl<'a> DataFrameTrait<'a> for DataFrame<'a> {}

pub trait DataFrameTrait<'a>: IEEE802_11FrameTrait<'a> {
  fn frag_seq(&self) -> u16 {
    u16::from(self.bytes()[22]) | ((u16::from(self.bytes()[23])) << 8)
  }

  /// Fragment Number
  fn fragment_number(&self) -> u8 {
    (self.frag_seq() & 0b0000_1111) as u8
  }

  /// Sequence Number
  fn sequence_number(&self) -> u16 {
    self.frag_seq() >> 4
  }
}
