const AUTHENTICATION_RESPONSE_PACKET: [u8; 38] = [
  0xb0, 0x00, 0x3a, 0x01, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x60, 0x1b, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xdd, 0x06,
  0x00, 0x10, 0x18, 0x01, 0x01, 0x00,
];

#[test]
fn test_authentication_response_packet() {
  // Receiver address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Destination address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Transmitter address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Source address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &AUTHENTICATION_RESPONSE_PACKET,
    subtype: Some(FrameSubtype::Management(ManagementSubtype::Authentication)),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(314)),

    receiver_address: "00:16:bc:3d:aa:57".parse().unwrap(),
    destination_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(438),

    ..Default::default()
  });

  let frame = Frame::new(&AUTHENTICATION_RESPONSE_PACKET);
  match match frame.next_layer().unwrap() {
    FrameLayer::Management(ref management_frame) => management_frame.next_layer().unwrap(),
    _ => unreachable!("not management"),
  } {
    ManagementFrameLayer::Authentication(ref authentication_frame) => {
      assert_eq!(
        authentication_frame.authentication_algorithm(),
        AuthenticationAlgorithm::OpenSystem,
        "authentication_algorithm"
      );
      assert_eq!(
        authentication_frame.authentication_seq(),
        0x0002,
        "authentication_seq"
      );
      assert_eq!(
        authentication_frame.status_code(),
        StatusCode::Successful,
        "status_code"
      );

      assert_eq!(
        authentication_frame
          .tagged_parameters()
          .get_bytes(TagName::Other(221))
          .unwrap(),
        [0x00, 0x10, 0x18, 0x01, 0x01, 0x00],
        "tagged_paramters"
      );
    }
    _ => unreachable!("not authentication"),
  }
}
