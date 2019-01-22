mod beacon;
mod fixed_parameters;
mod probe_request;
mod probe_response;
mod tagged_parameters;

pub use self::beacon::*;
pub use self::fixed_parameters::*;
pub use self::probe_request::*;
pub use self::probe_response::*;
pub use self::tagged_parameters::*;
use super::*;

pub struct ManagementFrame<'a> {
  bytes: &'a [u8],
}

pub enum ManagementFrameLayer<'a> {
  Beacon(BeaconFrame<'a>),
  ProbeRequest(ProbeRequestFrame<'a>),
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
        ManagementSubtype::ProbeRequest => Some(ManagementFrameLayer::ProbeRequest(
          ProbeRequestFrame::new(&self.bytes),
        )),
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

  fn transmitter_address(&self) -> Option<MacAddress> {
    Some(self.addr2())
  }

  fn bssid_address(&self) -> Option<MacAddress> {
    Some(self.addr3())
  }
}
impl<'a> FragmentSequenceTrait<'a> for ManagementFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for ManagementFrame<'a> {}

pub trait ManagementFrameTrait<'a>: FragmentSequenceTrait<'a> {}
