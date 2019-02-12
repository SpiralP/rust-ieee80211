use super::*;

const DEAUTHENTICATION_FRAME_SIZE: usize = DeauthenticationFrame::FRAGMENT_SEQUENCE_END;

pub struct DeauthenticationFrameBuilder {
  bytes: [u8; DEAUTHENTICATION_FRAME_SIZE],
}
impl DeauthenticationFrameBuilder {
  pub fn new_blank() -> Self {
    let mut frame = Self {
      bytes: [0; DEAUTHENTICATION_FRAME_SIZE],
    };

    frame.type_(FrameType::Management);
    frame.subtype(FrameSubtype::Management(
      ManagementSubtype::Deauthentication,
    ));

    frame
  }

  pub fn new_defaults() -> Self {
    Self::new_blank()
  }

  pub fn build(&self) -> DeauthenticationFrame {
    DeauthenticationFrame::new(self.bytes())
  }
}
impl FrameBuilderTrait for DeauthenticationFrameBuilder {
  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }
}
impl FragmentSequenceBuilderTrait for DeauthenticationFrameBuilder {}
impl ManagementFrameBuilderTrait for DeauthenticationFrameBuilder {}

#[test]
fn test_deauthentication_frame_builder() {
  let mut deauthentication_frame_builder = DeauthenticationFrameBuilder::new_blank();

  deauthentication_frame_builder.version(FrameVersion::Standard);
  deauthentication_frame_builder.subtype(FrameSubtype::Management(ManagementSubtype::Beacon));
  deauthentication_frame_builder.ds_status(DSStatus::FromDSToSTA);

  deauthentication_frame_builder.more_fragments(false);
  deauthentication_frame_builder.retry(true);
  deauthentication_frame_builder.pwr_mgt(true);
  deauthentication_frame_builder.more_data(false);
  deauthentication_frame_builder.protected(false);
  deauthentication_frame_builder.order(false);

  deauthentication_frame_builder.source_address("11:22:33:44:55:66".parse().unwrap());
  deauthentication_frame_builder.destination_address("22:22:33:44:55:66".parse().unwrap());
  deauthentication_frame_builder.bssid_address("33:22:33:44:55:66".parse().unwrap());

  deauthentication_frame_builder.sequence_number(10);
  deauthentication_frame_builder.fragment_number(11);

  let deauthentication_frame = deauthentication_frame_builder.build();

  assert_eq!(
    deauthentication_frame.version(),
    FrameVersion::Standard,
    "version"
  );
  assert_eq!(
    deauthentication_frame.subtype(),
    FrameSubtype::Management(ManagementSubtype::Beacon),
    "subtype"
  );
  assert_eq!(
    deauthentication_frame.ds_status(),
    DSStatus::FromDSToSTA,
    "ds_status"
  );

  assert_eq!(
    deauthentication_frame.more_fragments(),
    false,
    "more_fragments"
  );
  assert_eq!(deauthentication_frame.retry(), true, "retry");
  assert_eq!(deauthentication_frame.pwr_mgt(), true, "pwr_mgt");
  assert_eq!(
    deauthentication_frame.more_data(),
    false,
    "more_deauthentication"
  );
  assert_eq!(deauthentication_frame.protected(), false, "protected");
  assert_eq!(deauthentication_frame.order(), false, "order");

  assert_eq!(
    deauthentication_frame
      .source_address()
      .expect("source_address"),
    "11:22:33:44:55:66".parse().unwrap(),
    "source_address"
  );

  assert_eq!(
    deauthentication_frame
      .destination_address()
      .expect("destination_address"),
    "22:22:33:44:55:66".parse().unwrap(),
    "destination_address"
  );

  assert_eq!(
    deauthentication_frame
      .bssid_address()
      .expect("bssid_address"),
    "33:22:33:44:55:66".parse().unwrap(),
    "bssid_address"
  );

  assert_eq!(deauthentication_frame.sequence_number(), 10);
  assert_eq!(deauthentication_frame.fragment_number(), 11);
}