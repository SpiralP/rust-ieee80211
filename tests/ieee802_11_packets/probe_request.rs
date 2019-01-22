const IEEE802_11_PROBE_REQUEST: [u8; 54] = [
  0x40, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x40, 0x00, 0x00, 0x09, 0x6d, 0x61, 0x72, 0x74, 0x69, 0x6e,
  0x65, 0x74, 0x33, 0x01, 0x08, 0x82, 0x84, 0x8b, 0x96, 0x0c, 0x12, 0x18, 0x24, 0x03, 0x01, 0x0d,
  0x32, 0x04, 0x30, 0x48, 0x60, 0x6c,
];

#[test]
fn test_ieee802_11_probe_request() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Destination address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // Source address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)
  // BSS Id: Broadcast (ff:ff:ff:ff:ff:ff)

  test_test_item(TestItem {
    bytes: &IEEE802_11_PROBE_REQUEST,
    subtype: FrameSubtype::Management(ManagementSubtype::ProbeRequest),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(0),

    receiver_address: "ff:ff:ff:ff:ff:ff".parse().unwrap(),
    destination_address: Some("ff:ff:ff:ff:ff:ff".parse().unwrap()),

    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),
    source_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    bssid_address: Some("ff:ff:ff:ff:ff:ff".parse().unwrap()),
    station_address: None,

    fragment_number: Some(0),
    sequence_number: Some(4),

    ssid: Some(b"martinet3".to_vec()),
  });
}
