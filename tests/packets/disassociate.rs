const DISASSOCIATE_PACKET: [u8; 30] = [
  0xa0, 0x00, 0x3a, 0x01, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x00, 0x0d, 0x93, 0x82, 0x36, 0x3a,
  0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x50, 0x0b, 0x08, 0x00, 0xfe, 0xaa, 0x65, 0xac,
];

#[test]
fn test_disassociate_packet() {
  // Receiver address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // Destination address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // Transmitter address: Apple_82:36:3a (00:0d:93:82:36:3a)
  // Source address: Apple_82:36:3a (00:0d:93:82:36:3a)
  // BSS Id: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)

  test_test_item(TestItem {
    bytes: &DISASSOCIATE_PACKET,
    subtype: Some(FrameSubtype::Management(ManagementSubtype::Disassociate)),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(314)),

    receiver_address: "00:0c:41:82:b2:55".parse().unwrap(),
    destination_address: Some("00:0c:41:82:b2:55".parse().unwrap()),

    transmitter_address: Some("00:0d:93:82:36:3a".parse().unwrap()),
    source_address: Some("00:0d:93:82:36:3a".parse().unwrap()),

    bssid_address: Some("00:0c:41:82:b2:55".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(181),

    ..Default::default()
  });

  let frame = Frame::new(&DISASSOCIATE_PACKET[..]);
  match match frame.next_layer().unwrap() {
    FrameLayer::Management(ref management_frame) => management_frame.next_layer().unwrap(),
    _ => unreachable!("not management"),
  } {
    ManagementFrameLayer::Disassociate(ref disassociate_frame) => {
      // Reason code: Disassociated because sending STA is leaving (or has left) BSS (0x0008)
      assert_eq!(
        disassociate_frame.reason_code(),
        ReasonCode::STALeavingBSS,
        "reason_code"
      );
    }
    _ => unreachable!("not disassociate"),
  }
}
