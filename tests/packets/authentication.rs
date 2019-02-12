const AUTHENTICATION_PACKET: [u8; 30] = [
  0xb0, 0x00, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0xd0, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
];

#[test]
fn test_authentication_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Destination address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &AUTHENTICATION_PACKET,
    subtype: Some(FrameSubtype::Management(ManagementSubtype::Authentication)),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(258)),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    destination_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(13),

    ..Default::default()
  });

  let frame = Frame::new(&AUTHENTICATION_PACKET);
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
        0x0001,
        "authentication_seq"
      );
      assert_eq!(
        authentication_frame.status_code(),
        StatusCode::Successful,
        "status_code"
      );
    }
    _ => unreachable!("not authentication"),
  }
}
