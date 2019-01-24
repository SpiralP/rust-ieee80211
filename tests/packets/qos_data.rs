const QOS_DATA_PACKET: [u8; 97] = [
  0x88, 0x01, 0x2c, 0x00, 0x00, 0x14, 0xa5, 0xcd, 0x74, 0x7b, 0x00, 0x14, 0xa5, 0xcb, 0x6e, 0x1a,
  0x00, 0x01, 0x02, 0x27, 0xf9, 0xb2, 0xa0, 0xed, 0x00, 0x00, 0xaa, 0xaa, 0x03, 0x00, 0x00, 0x00,
  0x08, 0x00, 0x45, 0x00, 0x00, 0x3b, 0x8d, 0x06, 0x00, 0x00, 0x80, 0x11, 0x29, 0xd6, 0xc0, 0xa8,
  0x01, 0x84, 0xc0, 0xa8, 0x01, 0x01, 0x04, 0x07, 0x00, 0x35, 0x00, 0x27, 0xab, 0x15, 0x96, 0xc1,
  0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77, 0x77, 0x06, 0x70,
  0x6f, 0x6c, 0x69, 0x74, 0x6f, 0x02, 0x69, 0x74, 0x00, 0x00, 0x01, 0x00, 0x01, 0x78, 0x80, 0x59,
  0x37,
];

#[test]
fn test_qos_data_packet() {
  // Receiver address: GemtekTe_cd:74:7b (00:14:a5:cd:74:7b)
  // Transmitter address: GemtekTe_cb:6e:1a (00:14:a5:cb:6e:1a)
  // Destination address: 3Com_27:f9:b2 (00:01:02:27:f9:b2)
  // Source address: GemtekTe_cb:6e:1a (00:14:a5:cb:6e:1a)
  // BSS Id: GemtekTe_cd:74:7b (00:14:a5:cd:74:7b)
  // STA address: GemtekTe_cb:6e:1a (00:14:a5:cb:6e:1a)

  test_test_item(TestItem {
    bytes: &QOS_DATA_PACKET,
    subtype: Some(FrameSubtype::Data(DataSubtype::QoSData)),
    ds_status: Some(DSStatus::FromSTAToDS),

    duration_id: Some(DurationID::Duration(44)),

    receiver_address: "00:14:a5:cd:74:7b".parse().unwrap(),
    transmitter_address: Some("00:14:a5:cb:6e:1a".parse().unwrap()),

    destination_address: Some("00:01:02:27:f9:b2".parse().unwrap()),
    source_address: Some("00:14:a5:cb:6e:1a".parse().unwrap()),

    bssid_address: Some("00:14:a5:cd:74:7b".parse().unwrap()),
    station_address: Some("00:14:a5:cb:6e:1a".parse().unwrap()),

    fragment_number: Some(0),
    sequence_number: Some(3802),

    // QoS Control 0x0000
    ..Default::default()
  });
}
