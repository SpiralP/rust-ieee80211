use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait BeaconFixedParametersTrait: FrameTrait {
  // Fixed Parameters (24..36) on Beacons
  const FIXED_PARAMETERS_START: usize = 24;

  /// microseconds it has been active
  fn timestamp(&self) -> u64 {
    LittleEndian::read_u64(
      &self.bytes()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 8)],
    )
  }

  /// in seconds
  fn beacon_interval(&self) -> f64 {
    f64::from(LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 8)..(Self::FIXED_PARAMETERS_START + 10)],
    )) * 0.001_024_f64
  }

  fn capabilities_info(&self) -> CapabilitiesInfo {
    CapabilitiesInfo::from_bytes(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 10)..(Self::FIXED_PARAMETERS_START + 12)],
    )
  }
}

#[derive(Debug, PartialEq)]
pub struct CapabilitiesInfo {
  /// 0: Transmitter is a STA
  /// 1: Transmitter is an AP
  pub ess_capabilities: bool,

  /// 0: Transmitter belongs to a BSS
  /// 1: Transmitter belongs to an IBSS
  pub ibss_status: bool,

  // static const value_string sta_cf_pollable[] = {
  //   {0x00, "Station is not CF-Pollable"},
  //   {0x02, "Station is CF-Pollable, not requesting to be placed on the  CF-polling list"},
  //   {0x01, "Station is CF-Pollable, requesting to be placed on the CF-polling list"},
  //   {0x03, "Station is CF-Pollable, requesting never to be polled"},
  //   {0x80, "QSTA requesting association in QBSS"},
  //   {0x81, "Reserved"},
  //   {0x82, "Reserved"},
  //   {0x83, "Reserved"},
  //   {0, NULL}
  // };

  // static const value_string ap_cf_pollable[] = {
  //   {0x00, "No point coordinator at AP"},
  //   {0x02, "Point coordinator at AP for delivery only (no polling)"},
  //   {0x01, "Point coordinator at AP for delivery and polling"},
  //   {0x03, "Reserved"},
  //   {0x80, "QAP (HC) does not use CFP for delivery of unicast data type frames"},
  //   {0x82, "QAP (HC) uses CFP for delivery, but does not send CF-Polls to non-QoS STAs"},
  //   {0x81, "QAP (HC) uses CFP for delivery, and sends CF-Polls to non-QoS STAs"},
  //   {0x83, "Reserved"},
  //   {0, NULL}
  // };
  pub cfp_partitipation_capabilities: u8,

  /// 0: AP/STA cannot support WEP
  /// 1: AP/STA can support WEP
  pub privacy: bool,

  /// 0: Not Allowed
  pub short_preamble: bool,

  /// 0: Not Allowed
  pub pbcc: bool,

  /// 0: Not in use
  pub channel_agility: bool,

  /// 0: Not Implemented
  pub spectrum_management: bool,

  /// 1: In use
  pub short_slot_time: bool,

  /// 0: Not Implemented
  pub automatic_power_save_delivery: bool,

  /// 0: Not Implemented
  pub radio_measurement: bool,

  /// 0: Not Implemented
  pub dsss_ofdm: bool,

  /// 0: Not Implemented
  pub delayed_block_ack: bool,

  /// 0: Not Implemented
  pub immediate_block_ack: bool,
}
impl CapabilitiesInfo {
  #[must_use]
  pub fn from_bytes(bytes: &[u8]) -> Self {
    let b1 = bytes[0];
    let b2 = bytes[1];

    let n = u16::from(b1 & 0b0000_1100) | u16::from(b2 & 0b0000_0010);
    let cfp_partitipation_capabilities = (n >> 2) as u8;

    Self {
      ess_capabilities: (b1 & 0b0000_0001) != 0,
      ibss_status: (b1 & 0b0000_0010) != 0,
      cfp_partitipation_capabilities,
      privacy: (b1 & 0b0001_0000) != 0,
      short_preamble: (b1 & 0b0010_0000) != 0,
      pbcc: (b1 & 0b0100_0000) != 0,
      channel_agility: (b1 & 0b1000_0000) != 0,
      spectrum_management: (b2 & 0b0000_0001) != 0,
      short_slot_time: (b2 & 0b0000_0100) != 0,
      automatic_power_save_delivery: (b2 & 0b0000_1000) != 0,
      radio_measurement: (b2 & 0b0001_0000) != 0,
      dsss_ofdm: (b2 & 0b0010_0000) != 0,
      delayed_block_ack: (b2 & 0b0100_0000) != 0,
      immediate_block_ack: (b2 & 0b1000_0000) != 0,
    }
  }
}
