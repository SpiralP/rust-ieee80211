const BLOCK_ACK_PACKET: [u8; 28] = [
  0x94, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52,
  0x04, 0x00, 0xf0, 0xb3, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

#[test]
fn test_block_ack_packet() {
  // Receiver address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // Transmitter address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &BLOCK_ACK_PACKET,
    subtype: Some(FrameSubtype::Control(ControlSubtype::BlockAck)),

    receiver_address: "00:0c:41:82:b2:55".parse().unwrap(),
    transmitter_address: Some("00:15:00:34:18:52".parse().unwrap()),

    ..Default::default()
  });
}
