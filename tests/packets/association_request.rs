const ASSOCIATION_REQUEST_PACKET: [u8; 79] = [
  0x00, 0x00, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0xe0, 0x00, 0x11, 0x04, 0x0a, 0x00, 0x00, 0x09, 0x6d, 0x61,
  0x72, 0x74, 0x69, 0x6e, 0x65, 0x74, 0x33, 0x01, 0x08, 0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48,
  0x6c, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00,
  0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

#[test]
fn test_association_request_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Destination address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &ASSOCIATION_REQUEST_PACKET,
    subtype: Some(FrameSubtype::Management(
      ManagementSubtype::AssociationRequest,
    )),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(258)),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    destination_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(14),

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

  let frame = Frame::new(&ASSOCIATION_REQUEST_PACKET);
  match match frame.next_layer().unwrap() {
    FrameLayer::Management(ref management_frame) => management_frame.next_layer().unwrap(),
    _ => unreachable!("not management"),
  } {
    ManagementFrameLayer::AssociationRequest(ref association_request_frame) => {
      assert_eq!(
        association_request_frame.listen_interval(),
        0x000a,
        "listen_interval"
      );

      assert_eq!(
        association_request_frame.ssid().unwrap(),
        b"martinet3",
        "ssid"
      );
    }

    _ => unreachable!("not association request"),
  }
}
