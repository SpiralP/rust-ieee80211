const IEEE802_11_ACK_PACKET: [u8; 10] =
  [0xd4, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52];

#[test]
fn test_ieee802_11_ack_packet() {
  // Receiver address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &IEEE802_11_ACK_PACKET,
    subtype: Some(FrameSubtype::Control(ControlSubtype::Ack)),

    receiver_address: "00:15:00:34:18:52".parse().unwrap(),

    ..Default::default()
  });
}
