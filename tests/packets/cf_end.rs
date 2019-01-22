const CF_END_PACKET: [u8; 16] = [
  0xe4, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52,
];

#[test]
fn test_cf_end_packet() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &CF_END_PACKET,
    subtype: Some(FrameSubtype::Control(ControlSubtype::CFEnd)),

    receiver_address: "ff:ff:ff:ff:ff:ff".parse().unwrap(),

    bssid_address: Some("00:15:00:34:18:52".parse().unwrap()),

    ..Default::default()
  });
}
