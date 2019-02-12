mod association_request;
mod association_response;
mod authentication;
mod beacon;
mod builder;
mod deauthentication;
mod disassociate;
mod probe_request;
mod probe_response;
mod tagged_parameters;

pub use self::association_request::*;
pub use self::association_response::*;
pub use self::authentication::*;
pub use self::beacon::*;
pub use self::builder::*;
pub use self::deauthentication::*;
pub use self::disassociate::*;
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
  Authentication(AuthenticationFrame<'a>),
  Deauthentication(DeauthenticationFrame<'a>),
  Disassociate(DisassociateFrame<'a>),
  AssociationRequest(AssociationRequestFrame<'a>),
  AssociationResponse(AssociationResponseFrame<'a>),
}

impl<'a> ManagementFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  pub fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
  }
  pub fn addr3(&self) -> MacAddress {
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
        ManagementSubtype::Authentication => Some(ManagementFrameLayer::Authentication(
          AuthenticationFrame::new(&self.bytes),
        )),
        ManagementSubtype::Deauthentication => Some(ManagementFrameLayer::Deauthentication(
          DeauthenticationFrame::new(&self.bytes),
        )),
        ManagementSubtype::Disassociate => Some(ManagementFrameLayer::Disassociate(
          DisassociateFrame::new(&self.bytes),
        )),
        ManagementSubtype::AssociationRequest => Some(ManagementFrameLayer::AssociationRequest(
          AssociationRequestFrame::new(&self.bytes),
        )),
        ManagementSubtype::AssociationResponse => Some(ManagementFrameLayer::AssociationResponse(
          AssociationResponseFrame::new(&self.bytes),
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
