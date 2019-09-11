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

pub use self::{
  association_request::*, association_response::*, authentication::*, beacon::*, builder::*,
  deauthentication::*, disassociate::*, probe_request::*, probe_response::*, tagged_parameters::*,
};
use super::*;
use bytes::Bytes;

pub struct ManagementFrame {
  bytes: Bytes,
}

pub enum ManagementFrameLayer {
  Beacon(BeaconFrame),
  ProbeRequest(ProbeRequestFrame),
  ProbeResponse(ProbeResponseFrame),
  Authentication(AuthenticationFrame),
  Deauthentication(DeauthenticationFrame),
  Disassociate(DisassociateFrame),
  AssociationRequest(AssociationRequestFrame),
  AssociationResponse(AssociationResponseFrame),
}

impl ManagementFrame {
  pub fn new(bytes: Bytes) -> Self {
    Self { bytes }
  }

  pub fn next_layer(&self) -> Option<ManagementFrameLayer> {
    match self.subtype() {
      FrameSubtype::Management(subtype) => match subtype {
        ManagementSubtype::Beacon => {
          Some(ManagementFrameLayer::Beacon(BeaconFrame::new(self.bytes())))
        }
        ManagementSubtype::ProbeRequest => Some(ManagementFrameLayer::ProbeRequest(
          ProbeRequestFrame::new(self.bytes()),
        )),
        ManagementSubtype::ProbeResponse => Some(ManagementFrameLayer::ProbeResponse(
          ProbeResponseFrame::new(self.bytes()),
        )),
        ManagementSubtype::Authentication => Some(ManagementFrameLayer::Authentication(
          AuthenticationFrame::new(self.bytes()),
        )),
        ManagementSubtype::Deauthentication => Some(ManagementFrameLayer::Deauthentication(
          DeauthenticationFrame::new(self.bytes()),
        )),
        ManagementSubtype::Disassociate => Some(ManagementFrameLayer::Disassociate(
          DisassociateFrame::new(self.bytes()),
        )),
        ManagementSubtype::AssociationRequest => Some(ManagementFrameLayer::AssociationRequest(
          AssociationRequestFrame::new(self.bytes()),
        )),
        ManagementSubtype::AssociationResponse => Some(ManagementFrameLayer::AssociationResponse(
          AssociationResponseFrame::new(self.bytes()),
        )),
        _ => None,
      },
      _ => unreachable!(),
    }
  }
}

impl FrameTrait for ManagementFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }
}
impl FragmentSequenceTrait for ManagementFrame {}
impl ManagementFrameTrait for ManagementFrame {}

pub trait ManagementFrameTrait: FrameTrait + FragmentSequenceTrait {
  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[10..16]).unwrap()
  }
  fn addr3(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[16..22]).unwrap()
  }

  /// Transmitter Address
  /// Who this packet came from wirelessly.
  fn transmitter_address(&self) -> Option<MacAddress> {
    Some(self.addr2())
  }

  /// Source Address
  /// Who the packet came from.
  fn source_address(&self) -> Option<MacAddress> {
    self.transmitter_address()
  }

  /// Basic Service Set Address (BSSID)
  /// Basic Service Set ID for Multicast.
  fn bssid_address(&self) -> Option<MacAddress> {
    Some(self.addr3())
  }

  /// Station Address
  fn station_address(&self) -> Option<MacAddress> {
    None
  }
}
