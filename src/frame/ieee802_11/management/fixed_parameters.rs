pub trait FixedParametersTrait<'a> {
  // Fixed Parameters (24..36) on Beacons

  fn timestamp(&self) -> u64 {
    // let timestamp: u64 = bytes.read_u64::<LE>().unwrap();
    // &self.bytes()[24..32];
    unimplemented!()
  }

  /// in seconds
  fn beacon_interval(&self) -> f64 {
    // self.bytes()[32..34]
    // let beacon_interval = f64::from(bytes.read_u16::<LE>().unwrap()) * 0.001_024f64;
    unimplemented!()
  }

  fn capabilities_info(&self) -> CapabilitiesInfo {
    // 34..36
    unimplemented!()
  }
}

pub struct CapabilitiesInfo {
  pub ess_capabilities: bool,             // 1: Transmitter is an AP
  pub ibss_status: bool,                  // 0: Transmitter belongs to a BSS
  pub cfp_partitipation_capabilities: u8, // 0: No point coordinator at AP
  pub wep_supported: bool,
  pub short_preamble: bool,
  pub pbcc: bool,
  pub channel_agility: bool,     // 0: Not in use
  pub spectrum_management: bool, // 0: Not Implemented
  pub short_slot_time: bool,     // 1: In use
  pub automatic_power_save_delivery: bool,
  pub radio_measurement: bool,
  pub dsss_ofdm: bool,
  pub delayed_block_ack: bool,
  pub immediate_block_ack: bool,
}
