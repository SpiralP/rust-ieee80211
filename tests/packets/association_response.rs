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

    duration_id: DurationID::Duration(314),

    receiver_address: "00:16:bc:3d:aa:57".parse().unwrap(),
    destination_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    transmitter_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(439),

    ..Default::default()
  });
}
