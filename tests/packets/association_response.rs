const ASSOCIATION_RESPONSE_PACKET: [u8; 54] = [
  0x10, 0x00, 0x3a, 0x01, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x70, 0x1b, 0x11, 0x04, 0x00, 0x00, 0x04, 0xc0, 0x01, 0x08,
  0x82, 0x84, 0x8b, 0x96, 0x24, 0x30, 0x48, 0x6c, 0x32, 0x04, 0x0c, 0x12, 0x18, 0x60, 0xdd, 0x06,
  0x00, 0x10, 0x18, 0x01, 0x01, 0x00,
];

#[test]
fn test_association_response_packet() {
  // Receiver address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Destination address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Transmitter address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Source address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &ASSOCIATION_RESPONSE_PACKET,
    subtype: Some(FrameSubtype::Management(
      ManagementSubtype::AssociationResponse,
    )),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(314)),

    receiver_address: "00:16:bc:3d:aa:57".parse().unwrap(),
    destination_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(439),

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

  let frame = Frame::new(&ASSOCIATION_RESPONSE_PACKET);
  match match frame.next_layer().unwrap() {
    FrameLayer::Management(ref management_frame) => management_frame.next_layer().unwrap(),
    _ => unreachable!("not management"),
  } {
    ManagementFrameLayer::AssociationResponse(ref association_response_frame) => {
      assert_eq!(
        association_response_frame.status_code(),
        StatusCode::Successful,
        "status_code"
      );

      assert_eq!(
        association_response_frame.association_id(),
        0x0004,
        "association_id"
      );
    }

    _ => unreachable!("not association response"),
  }
}
