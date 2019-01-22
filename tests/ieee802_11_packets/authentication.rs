const IEEE802_11_AUTHENTICATION_PACKET: [u8; 30] = [
  0xb0, 0x00, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0xd0, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
];

#[test]
fn test_ieee802_11_authentication_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Destination address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)

  test_test_item(TestItem {
    bytes: &IEEE802_11_AUTHENTICATION_PACKET,
    subtype: FrameSubtype::Management(ManagementSubtype::Authentication),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(258),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    destination_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),

    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    station_address: None,

    fragment_number: Some(0),
    sequence_number: Some(13),

    ssid: None,
  });
}
