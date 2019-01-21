const IEEE802_11_NULL_DATA_PACKET: [u8; 24] = [
  0x48, 0x11, 0x02, 0x01, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0xf0, 0x03,
];

#[test]
fn test_ieee802_11_null_data_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Destination address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // STA address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)

  test_test_item(TestItem {
    bytes: &IEEE802_11_NULL_DATA_PACKET,
    subtype: FrameSubtype::Data(DataSubtype::Null),
    ds_status: DSStatus::FromSTAToDS,

    more_fragments: false,
    retry: false,
    pwr_mgt: true,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(258),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    destination_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    station_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(63),
  });
}
