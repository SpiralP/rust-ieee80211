const IEEE802_11_RTS_PACKET: [u8; 16] = [
  0xb4, 0x00, 0x98, 0x00, 0x00, 0x0c, 0x41, 0x82, 0xb2, 0x55, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52,
];

#[test]
fn test_ieee802_11_rts_packet() {
  // Receiver address: Cisco-Li_82:b2:55 (00:0c:41:82:b2:55)
  // Transmitter address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &IEEE802_11_RTS_PACKET,
    subtype: FrameSubtype::Control(ControlSubtype::RTS),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(152),

    receiver_address: "00:0c:41:82:b2:55".parse().unwrap(),
    transmitter_address: Some("00:15:00:34:18:52".parse().unwrap()),

    destination_address: None,
    source_address: None,

    bssid_address: None,
    station_address: None,

    fragment_number: None,
    sequence_number: None,

    ssid: None,
  });
}
