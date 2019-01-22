const CTS_PACKET: [u8; 14] = [
  0xc4, 0x00, 0x68, 0x00, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x55, 0x09, 0xcb, 0x58,
];

#[test]
fn test_cts_packet() {
  // Receiver address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)

  test_test_item(TestItem {
    bytes: &CTS_PACKET,
    subtype: Some(FrameSubtype::Control(ControlSubtype::CTS)),

    duration_id: DurationID::Duration(104),

    receiver_address: "00:0c:41:82:b2:55".parse().unwrap(),

    ..Default::default()
  });
}
