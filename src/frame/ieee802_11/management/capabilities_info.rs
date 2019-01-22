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
