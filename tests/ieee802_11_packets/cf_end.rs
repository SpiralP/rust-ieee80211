const IEEE802_11_CF_END_PACKET: [u8; 16] = [
  0xe4, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x15, 0x00, 0x34, 0x18, 0x52,
];

#[test]
fn test_ieee802_11_cf_end_packet() {
  // Receiver address: Broadcast (ff:ff:ff:ff:ff:ff)
  // Transmitter address: IntelCor_34:18:52 (00:15:00:34:18:52)

  test_test_item(TestItem {
    bytes: &IEEE802_11_CF_END_PACKET,
    subtype: FrameSubtype::Control(ControlSubtype::CFEnd),
    ds_status: DSStatus::NotLeavingDSOrADHOC,

    more_fragments: false,
    retry: false,
    pwr_mgt: false,
    more_data: false,
    protected: false,
    order: false,

    duration_id: DurationID::Duration(0),

    receiver_address: "ff:ff:ff:ff:ff:ff".parse().unwrap(),
    transmitter_address: None,

    destination_address: None,
    source_address: None,

    bssid_address: Some("00:15:00:34:18:52".parse().unwrap()),
    station_address: None,

    fragment_number: None,
    sequence_number: None,

    ssid: None,
  });
}
