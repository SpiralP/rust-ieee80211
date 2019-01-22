const BEACON_PACKET: [u8; 110] = [
  0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x10, 0xf0, 0x84, 0x21, 0x1a, 0x69, 0x02, 0x00, 0x00, 0x00,
  0x64, 0x00, 0x11, 0x04, 0x00, 0x09, 0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33, 0x01,
  0x08, 0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x03, 0x01, 0x0b, 0x05, 0x04, 0x00, 0x01,
  0x00, 0x00, 0x2a, 0x01, 0x04, 0x2f, 0x01, 0x04, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06,
  0x00, 0x10, 0x18, 0x01, 0x01, 0x00, 0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50,
  0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

#[test]
fn test_beacon_packet() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Destination address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Source address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &BEACON_PACKET,
    subtype: Some(FrameSubtype::Management(ManagementSubtype::Beacon)),

    receiver_address: "FF:FF:FF:FF:FF:FF".parse().unwrap(),
    destination_address: Some("FF:FF:FF:FF:FF:FF".parse().unwrap()),

    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(3841),

    ssid: Some(b"martinet3".to_vec()),

    timestamp: Some(0x0000_0002_691a_2184u64),

    beacon_interval: Some(0.102_400f64),

    capabilities_info: Some(CapabilitiesInfo {
      ess_capabilities: true, //
      ibss_status: false,
      cfp_partitipation_capabilities: 0,
      privacy: true, //
      short_preamble: false,
      pbcc: false,
      channel_agility: false,
      spectrum_management: false,
      short_slot_time: true, //
      automatic_power_save_delivery: false,
      radio_measurement: false,
      dsss_ofdm: false,
      delayed_block_ack: false,
      immediate_block_ack: false,
    }),

    supported_rates: Some(vec![1.0, 2.0, 5.5, 11.0, 18.0, 24.0, 36.0, 54.0]),

    ..Default::default()
  });
}
