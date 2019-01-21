const IEEE802_11_POWER_SAVE_POLL_PACKET: [u8; 16] = [
  0xa4, 0x10, 0x01, 0xc0, 0x00, 0x01, 0xe3, 0x41, 0xbd, 0x6e, 0x00, 0x16, 0xbc, 0x3d, 0xaa, 0x57,
];

#[test]
fn test_ieee802_11_power_save_poll_packet() {
  // Receiver address: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // BSS Id: Siemens_41:bd:6e (00:01:e3:41:bd:6e)
  // Transmitter address: NokiaDan_3d:aa:57 (00:16:bc:3d:aa:57)

  test_test_item(TestItem {
    bytes: &IEEE802_11_POWER_SAVE_POLL_PACKET,
    subtype: FrameSubtype::Control(ControlSubtype::PSPoll),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: true, //
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::AssociationID(1),

    receiver_address: "00:01:e3:41:bd:6e".parse().unwrap(),
    transmitter_address: Some("00:16:bc:3d:aa:57".parse().unwrap()),

    destination_address: None,
    source_address: None,

    bssid_address: Some("00:01:e3:41:bd:6e".parse().unwrap()),
    station_address: None,

    fragment_number: None,
    sequence_number: None,
  });
}
