const BEACON_CIPHERS_PACKET: [u8; 140] = [
  0x80, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55,
  0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x50, 0xf8, 0x89, 0xf1, 0xd4, 0x1b, 0x01, 0x00, 0x00, 0x00,
  0x64, 0x00, 0x11, 0x04, 0x00, 0x07, 0x43, 0x6f, 0x68, 0x65, 0x72, 0x65, 0x72, 0x01, 0x08, 0x82,
  0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x03, 0x01, 0x01, 0x05, 0x04, 0x00, 0x01, 0x00, 0x00,
  0x2a, 0x01, 0x02, 0x2f, 0x01, 0x02, 0x30, 0x18, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x02, 0x02, 0x00,
  0x00, 0x0f, 0xac, 0x04, 0x00, 0x0f, 0xac, 0x02, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x02, 0x00, 0x00,
  0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06, 0x00, 0x10, 0x18, 0x02, 0x00, 0x04, 0xdd, 0x1c,
  0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x02, 0x00, 0x00, 0x50, 0xf2, 0x04,
  0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x00, 0x00,
];

#[test]
fn test_beacon_ciphers_packet() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Destination address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // Source address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // BSS Id: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)

  test_test_item(TestItem {
    bytes: &BEACON_CIPHERS_PACKET,
    subtype: Some(FrameSubtype::Management(ManagementSubtype::Beacon)),

    receiver_address: "FF:FF:FF:FF:FF:FF".parse().unwrap(),
    destination_address: Some("FF:FF:FF:FF:FF:FF".parse().unwrap()),

    transmitter_address: Some("00:0c:41:82:b2:55".parse().unwrap()),
    source_address: Some("00:0c:41:82:b2:55".parse().unwrap()),

    bssid_address: Some("00:0c:41:82:b2:55".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(3973),

    ssid: Some(b"Coherer".to_vec()),

    timestamp: Some(0x0000_0001_1bd4_f189u64),

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

    rsn: Some(RSNVersion::Standard(RSN {
      group_cipher_suite: Some(CipherSuite::Standard(CipherSuiteType::TKIP)),
      pairwise_cipher_suites: vec![
        CipherSuite::Standard(CipherSuiteType::CCMP),
        CipherSuite::Standard(CipherSuiteType::TKIP),
      ],
      akm_suites: vec![AKMSuite::Standard(AKMSuiteType::PSK)],
      capabilities: Some(RSNCapabilities {
        pre_auth: false,
        pairwise: false,
        ptksa_replay_counter_value: 0,
        gtksa_replay_counter_value: 0,
        management_frame_protection_required: false,
        management_frame_protection_capable: false,
        joint_multi_band_rsna: false,
        peerkey: false,
      }),
    })),

    channel: Some(1),

    ..Default::default()
  });
}
