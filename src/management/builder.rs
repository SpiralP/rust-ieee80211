use super::*;

// TODO dynamic size
const MANAGEMENT_FRAME_SIZE: usize = ManagementFrameBuilder::FRAGMENT_SEQUENCE_START + 2;

pub struct ManagementFrameBuilder {
  bytes: Vec<u8>,
}
impl ManagementFrameBuilder {
  pub fn new_blank() -> Self {
    Self {
      bytes: vec![0; MANAGEMENT_FRAME_SIZE],
    }
  }

  pub fn new_defaults() -> Self {
    Self::new_blank()
  }

  pub fn build(&self) -> ManagementFrame {
    ManagementFrame::new(self.bytes())
  }

  pub fn addr2(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[10..16].copy_from_slice(mac_address.as_bytes());
  }

  pub fn addr3(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[16..22].copy_from_slice(mac_address.as_bytes());
  }
}
impl FrameBuilderTrait for ManagementFrameBuilder {
  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }

  fn transmitter_address(&mut self, mac_address: MacAddress) {
    self.addr2(mac_address)
  }

  fn bssid_address(&mut self, mac_address: MacAddress) {
    self.addr3(mac_address)
  }
}

impl FragmentSequenceBuilderTrait for ManagementFrameBuilder {}

#[test]
fn test_management_frame_builder() {
  let mut management_frame_builder = ManagementFrameBuilder::new_blank();

  management_frame_builder.version(FrameVersion::Standard);
  management_frame_builder.type_(FrameType::Management);
  management_frame_builder.subtype(FrameSubtype::Management(ManagementSubtype::Beacon));
  management_frame_builder.ds_status(DSStatus::FromDSToSTA);

  management_frame_builder.more_fragments(false);
  management_frame_builder.retry(true);
  management_frame_builder.pwr_mgt(true);
  management_frame_builder.more_data(false);
  management_frame_builder.protected(false);
  management_frame_builder.order(false);

  management_frame_builder.source_address("11:22:33:44:55:66".parse().unwrap());
  management_frame_builder.destination_address("22:22:33:44:55:66".parse().unwrap());
  management_frame_builder.bssid_address("33:22:33:44:55:66".parse().unwrap());

  management_frame_builder.sequence_number(10);
  management_frame_builder.fragment_number(11);

  let management_frame = management_frame_builder.build();

  assert_eq!(
    management_frame.version(),
    FrameVersion::Standard,
    "version"
  );
  assert_eq!(management_frame.type_(), FrameType::Management, "type_");
  assert_eq!(
    management_frame.subtype(),
    FrameSubtype::Management(ManagementSubtype::Beacon),
    "subtype"
  );
  assert_eq!(
    management_frame.ds_status(),
    DSStatus::FromDSToSTA,
    "ds_status"
  );

  assert_eq!(management_frame.more_fragments(), false, "more_fragments");
  assert_eq!(management_frame.retry(), true, "retry");
  assert_eq!(management_frame.pwr_mgt(), true, "pwr_mgt");
  assert_eq!(management_frame.more_data(), false, "more_management");
  assert_eq!(management_frame.protected(), false, "protected");
  assert_eq!(management_frame.order(), false, "order");

  assert_eq!(
    management_frame.source_address().unwrap(),
    "11:22:33:44:55:66".parse().unwrap(),
    "source_address"
  );

  assert_eq!(
    management_frame
      .destination_address()
      .expect("destination_address"),
    "22:22:33:44:55:66".parse().unwrap(),
    "destination_address"
  );

  assert_eq!(
    management_frame.bssid_address().expect("bssid_address"),
    "33:22:33:44:55:66".parse().unwrap(),
    "bssid_address"
  );

  assert_eq!(management_frame.sequence_number(), 10);
  assert_eq!(management_frame.fragment_number(), 11);
}
