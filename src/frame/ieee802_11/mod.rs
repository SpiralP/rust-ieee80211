mod control;
mod data;
mod fragment_sequence;
mod management;
mod types;

pub use self::control::*;
pub use self::data::*;
pub use self::fragment_sequence::*;
pub use self::management::*;
pub use self::types::*;
use super::*;
use eui48::MacAddress;

pub struct IEEE802_11Frame<'a> {
  bytes: &'a [u8],
}

pub enum IEEE802_11FrameLayer<'a> {
  Management(ManagementFrame<'a>),
  Control(ControlFrame<'a>),
  Data(DataFrame<'a>),
}

impl<'a> IEEE802_11Frame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }

  pub fn next_layer(&self) -> IEEE802_11FrameLayer<'a> {
    match self.type_() {
      FrameType::Management => {
        IEEE802_11FrameLayer::Management(ManagementFrame::new(&self.bytes()))
      }
      FrameType::Control => IEEE802_11FrameLayer::Control(ControlFrame::new(&self.bytes())),
      FrameType::Data => IEEE802_11FrameLayer::Data(DataFrame::new(&self.bytes())),
    }
  }
}
impl<'a> FrameTrait<'a> for IEEE802_11Frame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> IEEE802_11FrameTrait<'a> for IEEE802_11Frame<'a> {}

pub trait IEEE802_11FrameTrait<'a>: FrameTrait<'a> {
  fn version(&self) -> FrameVersion {
    match self.bytes()[0] & 0b0000_0011 {
      0 => FrameVersion::Standard,
      _ => unreachable!(),
    }
  }

  /// Main IEEE 802.11 Frame Type
  fn type_(&self) -> FrameType {
    FrameType::from((self.bytes()[0] & 0b0000_1100) >> 2)
  }

  /// IEEE 802.11 Frame Subtype
  fn subtype(&self) -> FrameSubtype {
    let subtype = (self.bytes()[0] & 0b1111_0000) >> 4;

    FrameSubtype::from(self.type_(), subtype)
  }

  // flags

  /// to Distribution System
  fn to_ds(&self) -> bool {
    self.bytes()[1] & 0b0000_0001 != 0
  }

  /// from Distribution System
  #[allow(clippy::wrong_self_convention)]
  fn from_ds(&self) -> bool {
    self.bytes()[1] & 0b0000_0010 != 0
  }

  fn ds_status(&self) -> DSStatus {
    match (self.from_ds(), self.to_ds()) {
      // 00 Not leaving DS or network is operating in AD-HOC mode
      (false, false) => DSStatus::NotLeavingDSOrADHOC,
      // 01 Frame from STA to DS via an AP
      (false, true) => DSStatus::FromSTAToDS,
      // 10 Frame from DS to a STA via AP
      (true, false) => DSStatus::FromDSToSTA,
      // 11 WDS (AP to AP) or Mesh (MP to MP) Frame
      (true, true) => DSStatus::WDSOrMesh,
    }
  }

  /// 0: This is the last fragment
  /// 1: More fragments follow
  fn more_fragments(&self) -> bool {
    (self.bytes()[1] & 0b0000_0100) != 0
  }

  /// 0: Frame is not being retransmitted
  /// 1: Frame is being retransmitted
  fn retry(&self) -> bool {
    (self.bytes()[1] & 0b0000_1000) != 0
  }

  /// 0: STA will stay up
  /// 1: STA will go to sleep
  fn pwr_mgt(&self) -> bool {
    (self.bytes()[1] & 0b0001_0000) != 0
  }

  /// 0: No data buffered
  /// 1: Data is buffered for STA at AP
  fn more_data(&self) -> bool {
    (self.bytes()[1] & 0b0010_0000) != 0
  }

  /// 0: Data is not protected
  /// 1: Data is protected
  fn protected(&self) -> bool {
    (self.bytes()[1] & 0b0100_0000) != 0
  }

  /// 0: Not strictly ordered
  /// 1: Strictly ordered
  fn order(&self) -> bool {
    (self.bytes()[1] & 0b1000_0000) != 0
  }

  /// Duration or Association Identifier
  fn duration_or_id(&self) -> DurationID {
    if (self.bytes()[3] & 0b1000_0000) != 0 {
      let n = u16::from(self.bytes()[2]) | ((u16::from(self.bytes()[3]) & 0b0011_1111) << 8);
      // TODO valid range 1-2007, use Reserved
      DurationID::AssociationID(n)
    } else {
      let n = u16::from(self.bytes()[2]) | ((u16::from(self.bytes()[3]) & 0b0111_1111) << 8);
      DurationID::Duration(n)
    }
  }

  // Addressing

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
  }

  /// Receiver Address
  /// Who this packet is destined for wirelessly.
  /// Address 1
  fn receiver_address(&self) -> MacAddress {
    self.addr1()
  }

  /// Transmitter Address
  /// Who this packet came from wirelessly.
  fn transmitter_address(&self) -> Option<MacAddress> {
    None
  }

  /// Destination Address
  /// Who the packet is destined for.
  fn destination_address(&self) -> Option<MacAddress> {
    Some(self.receiver_address())
  }

  /// Source Address
  /// Who the packet came from.
  fn source_address(&self) -> Option<MacAddress> {
    self.transmitter_address()
  }

  /// Basic Service Set Address (BSSID)
  /// Basic Service Set ID for Multicast.
  fn bssid_address(&self) -> Option<MacAddress> {
    None
  }

  /// Station Address
  fn station_address(&self) -> Option<MacAddress> {
    None
  }
}
