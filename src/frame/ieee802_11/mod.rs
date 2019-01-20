mod types;

pub use self::types::*;
use byteorder::LittleEndian;

pub struct IEEE802_11Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> IEEE802_11Frame<'a> {
  pub fn new(bytes: &'a [u8]) -> IEEE802_11Frame<'a> {
    IEEE802_11Frame { bytes }
  }

  pub fn version(&self) -> IEEE802_11FrameVersion {
    match self.bytes[0] & 0b0000_0011 {
      0 => IEEE802_11FrameVersion::Standard,
      _ => unreachable!(),
    }
  }

  /// Main IEEE 802.11 Frame Type
  pub fn type_(&self) -> IEEE802_11FrameType {
    // TODO transmute?
    match (self.bytes[0] & 0b0000_1100) >> 2 {
      0 => IEEE802_11FrameType::Management,
      1 => IEEE802_11FrameType::Control,
      2 => IEEE802_11FrameType::Data,
      3 => IEEE802_11FrameType::Extension,
      _ => unreachable!(),
    }
  }

  /// IEEE 802.11 Frame Subtype
  pub fn subtype(&self) -> IEEE802_11FrameSubtype {
    let subtype = (self.bytes[0] & 0b1111_0000) >> 4;
    match self.type_() {
      IEEE802_11FrameType::Management => {
        IEEE802_11FrameSubtype::Management(ManagementSubtype::from(subtype))
      }
      IEEE802_11FrameType::Control => {
        IEEE802_11FrameSubtype::Control(ControlSubtype::from(subtype))
      }
      IEEE802_11FrameType::Data => IEEE802_11FrameSubtype::Data(DataSubtype::from(subtype)),
      IEEE802_11FrameType::Extension => IEEE802_11FrameSubtype::Extension,
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
    match (self.to_ds(), self.from_ds()) {
      // 00 Not leaving DS or network is operating in AD-HOC mode
      (false, false) => DSStatus::NotLeavingDSOrADHOC,
      // 10 Frame from DS to a STA via AP
      (true, false) => DSStatus::FromDSToSTA,
      // 01 Frame from STA to DS via an AP
      (false, true) => DSStatus::FromSTAtoDS,
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

  /// Duration/ID
  pub fn duration_or_id(&self) -> DurationID {
    if (self.bytes[3] & 0b1000_0000) != 0 {
      let n = u16::from(self.bytes[2]) | ((u16::from(self.bytes[3]) & 0b0011_1111) << 8);
      DurationID::AssociationID(n)
    } else {
      let n = u16::from(self.bytes[2]) | ((u16::from(self.bytes[3]) & 0b0111_1111) << 8);
      DurationID::Duration(n)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::ieee802_11::*;

  const IEEE802_11_BEACON_PACKET: [u8; 110] = [
    0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
    0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x20, 0xf9, 0x88, 0xb1, 0xfc, 0x69, 0x02, 0x00, 0x00, 0x00,
    0x64, 0x00, 0x11, 0x04, 0x00, 0x09, 0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33, 0x01,
    0x08, 0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x03, 0x01, 0x0b, 0x05, 0x04, 0x00, 0x01,
    0x00, 0x00, 0x2a, 0x01, 0x04, 0x2f, 0x01, 0x04, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06,
    0x00, 0x10, 0x18, 0x01, 0x01, 0x00, 0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50,
    0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
  ];

  #[test]
  fn test_ieee802_11_beacon_packet() {
    let frame = IEEE802_11Frame::new(&IEEE802_11_BEACON_PACKET);

    assert_eq!(
      frame.subtype(),
      IEEE802_11FrameSubtype::Management(ManagementSubtype::Beacon)
    );

    assert_eq!(frame.ds_status(), DSStatus::NotLeavingDSOrADHOC);
    assert!(
      !(frame.more_fragments()
        || frame.retry()
        || frame.pwr_mgt()
        || frame.more_data()
        || frame.protected()
        || frame.order())
    );

    assert_eq!(frame.duration_or_id(), DurationID::Duration(0));
  }

  const IEEE802_11_AUTHENTICATION_PACKET: [u8; 30] = [
    0xb0, 0x00, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
    0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0xd0, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
  ];

  #[test]
  fn test_ieee802_11_authentication_packet() {
    let frame = IEEE802_11Frame::new(&IEEE802_11_AUTHENTICATION_PACKET);

    assert_eq!(
      frame.subtype(),
      IEEE802_11FrameSubtype::Management(ManagementSubtype::Authentication)
    );

    assert_eq!(frame.ds_status(), DSStatus::NotLeavingDSOrADHOC);
    assert!(
      !(frame.more_fragments()
        || frame.retry()
        || frame.pwr_mgt()
        || frame.more_data()
        || frame.protected()
        || frame.order())
    );

    assert_eq!(frame.duration_or_id(), DurationID::Duration(258));
  }

  const IEEE802_11_POWER_SAVE_POLL_PACKET: [u8; 16] = [
    0xa4, 0x10, 0x01, 0xc0, 0x00, 0x1b, 0x2f, 0x09, 0x53, 0xc2, 0x00, 0x21, 0xe9, 0xe3, 0x58, 0x4d,
  ];

  #[test]
  fn test_ieee802_11_power_save_poll_packet() {
    let frame = IEEE802_11Frame::new(&IEEE802_11_POWER_SAVE_POLL_PACKET);

    assert_eq!(
      frame.subtype(),
      IEEE802_11FrameSubtype::Control(ControlSubtype::PSPoll)
    );

    assert_eq!(frame.ds_status(), DSStatus::NotLeavingDSOrADHOC);
    assert!(frame.pwr_mgt());
    assert!(
      !(frame.more_fragments()
        || frame.retry()
        || frame.more_data()
        || frame.protected()
        || frame.order())
    );

    assert_eq!(frame.duration_or_id(), DurationID::AssociationID(1));
  }

}
