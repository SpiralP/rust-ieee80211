mod types;

pub use self::types::*;
use eui48::MacAddress;

pub struct IEEE802_11Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> IEEE802_11Frame<'a> {
  pub fn new(bytes: &'a [u8]) -> IEEE802_11Frame<'a> {
    IEEE802_11Frame { bytes }
  }

  pub fn version(&self) -> FrameVersion {
    match self.bytes[0] & 0b0000_0011 {
      0 => FrameVersion::Standard,
      _ => unreachable!(),
    }
  }

  /// Main IEEE 802.11 Frame Type
  pub fn type_(&self) -> FrameType {
    // TODO transmute to enum?
    match (self.bytes[0] & 0b0000_1100) >> 2 {
      0 => FrameType::Management,
      1 => FrameType::Control,
      2 => FrameType::Data,
      3 => FrameType::Extension,
      _ => unreachable!(),
    }
  }

  /// IEEE 802.11 Frame Subtype
  pub fn subtype(&self) -> FrameSubtype {
    let subtype = (self.bytes[0] & 0b1111_0000) >> 4;
    match self.type_() {
      FrameType::Management => FrameSubtype::Management(ManagementSubtype::from(subtype)),
      FrameType::Control => FrameSubtype::Control(ControlSubtype::from(subtype)),
      FrameType::Data => FrameSubtype::Data(DataSubtype::from(subtype)),
      FrameType::Extension => FrameSubtype::Extension,
    }
  }

  // flags

  /// to Distribution System
  pub fn to_ds(&self) -> bool {
    self.bytes[1] & 0b0000_0001 != 0
  }

  /// from Distribution System
  #[allow(clippy::wrong_self_convention)]
  pub fn from_ds(&self) -> bool {
    self.bytes[1] & 0b0000_0010 != 0
  }

  pub fn ds_status(&self) -> DSStatus {
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
  pub fn more_fragments(&self) -> bool {
    (self.bytes[1] & 0b0000_0100) != 0
  }

  /// 0: Frame is not being retransmitted
  /// 1: Frame is being retransmitted
  pub fn retry(&self) -> bool {
    (self.bytes[1] & 0b0000_1000) != 0
  }

  /// 0: STA will stay up
  /// 1: STA will go to sleep
  pub fn pwr_mgt(&self) -> bool {
    (self.bytes[1] & 0b0001_0000) != 0
  }

  /// 0: No data buffered
  /// 1: Data is buffered for STA at AP
  pub fn more_data(&self) -> bool {
    (self.bytes[1] & 0b0010_0000) != 0
  }

  /// 0: Data is not protected
  /// 1: Data is protected
  pub fn protected(&self) -> bool {
    (self.bytes[1] & 0b0100_0000) != 0
  }

  /// 0: Not strictly ordered
  /// 1: Strictly ordered
  pub fn order(&self) -> bool {
    (self.bytes[1] & 0b1000_0000) != 0
  }

  /// Duration or Association Identifier
  pub fn duration_or_id(&self) -> DurationID {
    if (self.bytes[3] & 0b1000_0000) != 0 {
      let n = u16::from(self.bytes[2]) | ((u16::from(self.bytes[3]) & 0b0011_1111) << 8);
      // TODO valid range 1-2007, use Reserved
      DurationID::AssociationID(n)
    } else {
      let n = u16::from(self.bytes[2]) | ((u16::from(self.bytes[3]) & 0b0111_1111) << 8);
      DurationID::Duration(n)
    }
  }

  fn addr1(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[4..10]).unwrap()
  }
  fn addr2(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[10..16]).unwrap()
  }
  fn addr3(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[16..22]).unwrap()
  }
  fn addr4(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[22..28]).unwrap()
  }

  /// Receiver Address
  /// Who this packet is destined for wirelessly.
  /// Address 1
  pub fn receiver_address(&self) -> MacAddress {
    self.addr1()
  }

  /// Transmitter Address
  /// Who this packet came from wirelessly.
  pub fn transmitter_address(&self) -> Option<MacAddress> {
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

  /// Destination Address
  /// Who the packet is destined for.
  pub fn destination_address(&self) -> Option<MacAddress> {
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

  /// Source Address
  /// Who the packet came from.
  pub fn source_address(&self) -> Option<MacAddress> {
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
  pub fn bssid_address(&self) -> Option<MacAddress> {
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
  pub fn station_address(&self) -> Option<MacAddress> {
    match self.type_() {
      FrameType::Data => match self.ds_status() {
        DSStatus::FromDSToSTA => Some(self.addr1()),
        DSStatus::FromSTAToDS => Some(self.addr2()),
        _ => None,
      },
      _ => None,
    }
  }

  pub fn fragment_number(&self) -> Option<u8> {
    // TODO
    None
  }

  pub fn sequence_number(&self) -> Option<u16> {
    // TODO
    None
  }
}
