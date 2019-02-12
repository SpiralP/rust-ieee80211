const DEAUTHENTICATION_PACKET: [u8; 26] = [
  0xc0, 0x00, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x80, 0x04, 0x03, 0x00,
];

#[test]
fn test_deauthentication_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Destination address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &DEAUTHENTICATION_PACKET,
    subtype: Some(FrameSubtype::Management(
      ManagementSubtype::Deauthentication,
    )),

    ds_status: Some(DSStatus::NotLeavingDSOrADHOC),

    duration_id: Some(DurationID::Duration(258)),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    destination_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(72),

    ..Default::default()
  });

  let frame = Frame::new(&DEAUTHENTICATION_PACKET);
  match match frame.next_layer().unwrap() {
    FrameLayer::Management(ref management_frame) => management_frame.next_layer().unwrap(),
    _ => unreachable!("not management"),
  } {
    ManagementFrameLayer::Deauthentication(ref deauthentication_frame) => {
      // Reason code: Deauthenticated because sending STA is leaving (or has left) IBSS or ESS (0x0003)
      assert_eq!(
        deauthentication_frame.reason_code(),
        ReasonCode::STALeavingIBSSOrESS,
        "reason_code"
      );
    }
    _ => unreachable!("not deauthentication"),
  }
}
