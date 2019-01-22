mod beacon;
mod capabilities_info;
mod fixed_tagged_params;
mod probe_response;
mod tagged_params;

pub use self::beacon::*;
pub use self::capabilities_info::*;
pub use self::fixed_tagged_params::*;
pub use self::probe_response::*;
pub use self::tagged_params::*;
use super::*;

pub struct ManagementFrame<'a> {
  bytes: &'a [u8],
}

pub enum ManagementFrameLayer<'a> {
  Beacon(BeaconFrame<'a>),
  ProbeResponse(ProbeResponseFrame<'a>),
}

impl<'a> ManagementFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
  }
  fn addr3(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[16..22]).unwrap()
  }

  pub fn next_layer(&self) -> Option<ManagementFrameLayer<'a>> {
    match self.subtype() {
      FrameSubtype::Management(subtype) => match subtype {
        ManagementSubtype::Beacon => {
          Some(ManagementFrameLayer::Beacon(BeaconFrame::new(&self.bytes)))
        }
        ManagementSubtype::ProbeResponse => Some(ManagementFrameLayer::ProbeResponse(
          ProbeResponseFrame::new(&self.bytes),
        )),
        _ => None,
      },
      _ => unreachable!(),
    }
  }
}
impl<'a> FrameTrait<'a> for ManagementFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> IEEE802_11FrameTrait<'a> for ManagementFrame<'a> {
  fn transmitter_address(&self) -> Option<MacAddress> {
    Some(self.addr2())
  }

  fn bssid_address(&self) -> Option<MacAddress> {
    Some(self.addr3())
  }
}
impl<'a> ManagementFrameTrait<'a> for ManagementFrame<'a> {}

pub trait ManagementFrameTrait<'a>: IEEE802_11FrameTrait<'a> {
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
