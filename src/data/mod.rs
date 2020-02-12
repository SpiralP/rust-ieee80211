mod builder;

pub use self::builder::*;
use super::*;
use bytes::Bytes;

pub struct DataFrame {
  bytes: Bytes,
}

impl DataFrame {
  pub fn new<T: Into<Bytes>>(bytes: T) -> Self {
    Self {
      bytes: bytes.into(),
    }
  }

  pub fn next_layer(&self) -> Option<Bytes> {
    let mut index = Self::FRAGMENT_SEQUENCE_START + 2;

    if self.protected() {
      index += 8; // skip TKIP/CCMP parameters
    }

    match self.subtype() {
      FrameSubtype::Data(ref subtype) => match subtype {
        DataSubtype::QoSData => {
          index += 2; // skip Qos Control
        }
        DataSubtype::Data => {}
        _ => return None,
      },
      _ => unreachable!(),
    }

    Some(self.bytes().slice(index..))
  }
}

impl FrameTrait for DataFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
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
}
impl FragmentSequenceTrait for DataFrame {}
impl DataFrameTrait for DataFrame {}

pub trait DataFrameTrait: FrameTrait {
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

  fn transmitter_address(&self) -> Option<MacAddress> {
    Some(self.addr2())
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

  fn qos_control(&self) -> u8 {
    // self.bytes()[(Self::FRAGMENT_SEQUENCE_START + 2)..];
    unimplemented!()
  }
}
