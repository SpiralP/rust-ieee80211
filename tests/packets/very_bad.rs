const VERY_BAD_PACKET: [u8; 10] = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0, 0, 0, 0];

#[test]
fn test_very_bad_packet() {
  test_test_item(TestItem {
    bytes: &VERY_BAD_PACKET,

    ds_status: Some(DSStatus::WDSOrMesh),

    duration_id: Some(DurationID::Reserved(16383)),

    version: Some(FrameVersion::Reserved(3)),

    type_: Some(FrameType::Reserved(3)),

    subtype: Some(FrameSubtype::Reserved(3, 15)),

    more_fragments: true,
    retry: true,
    pwr_mgt: true,
    more_data: true,
    protected: true,
    order: true,

    receiver_address: "ff:ff:00:00:00:00".parse().unwrap(),

    ..Default::default()
  });
}
