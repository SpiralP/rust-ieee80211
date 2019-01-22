const IEEE802_11_CTS_PACKET: [u8; 14] = [
  0xc4, 0x00, 0x68, 0x00, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x55, 0x09, 0xcb, 0x58,
];

#[test]
fn test_ieee802_11_cts_packet() {
  // Receiver address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)

  test_test_item(TestItem {
    bytes: &IEEE802_11_CTS_PACKET,
    subtype: FrameSubtype::Control(ControlSubtype::CTS),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(104),

    receiver_address: "00:0c:41:82:b2:55".parse().unwrap(),
    transmitter_address: None,

    destination_address: None,
    source_address: None,

    bssid_address: None,
    station_address: None,

    fragment_number: None,
    sequence_number: None,

    ssid: None,
  });
}
