use super::*;

const DISASSOCIATE_FRAME_SIZE: usize = DisassociateFrame::FIXED_PARAMETERS_END;

#[derive(Default)]
pub struct DisassociateFrameBuilder {
  bytes: [u8; DISASSOCIATE_FRAME_SIZE],
}
impl DisassociateFrameBuilder {
  pub fn new() -> Self {
    let mut builder = Self {
      bytes: [0; DISASSOCIATE_FRAME_SIZE],
    };

    builder.type_(FrameType::Management);
    builder.subtype(FrameSubtype::Management(ManagementSubtype::Disassociate));

    builder.reason_code(ReasonCode::STALeavingBSS);

    builder
  }

  pub fn build(&self) -> DisassociateFrame {
    DisassociateFrame::new(self.bytes().to_vec())
  }
}
impl FrameBuilderTrait for DisassociateFrameBuilder {
  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }
}
impl FragmentSequenceBuilderTrait for DisassociateFrameBuilder {}
impl ManagementFrameBuilderTrait for DisassociateFrameBuilder {}
impl DisassociateFixedParametersBuilderTrait for DisassociateFrameBuilder {}

#[test]
fn test_disassociate_frame_builder() {
  let mut disassociate_frame_builder = DisassociateFrameBuilder::new();

  disassociate_frame_builder.version(FrameVersion::Standard);
  disassociate_frame_builder.subtype(FrameSubtype::Management(ManagementSubtype::Beacon));
  disassociate_frame_builder.ds_status(DSStatus::FromDSToSTA);

  disassociate_frame_builder.more_fragments(false);
  disassociate_frame_builder.retry(true);
  disassociate_frame_builder.pwr_mgt(true);
  disassociate_frame_builder.more_data(false);
  disassociate_frame_builder.protected(false);
  disassociate_frame_builder.order(false);

  disassociate_frame_builder.source_address("11:22:33:44:55:66".parse().unwrap());
  disassociate_frame_builder.destination_address("22:22:33:44:55:66".parse().unwrap());
  disassociate_frame_builder.bssid_address("33:22:33:44:55:66".parse().unwrap());

  disassociate_frame_builder.sequence_number(10);
  disassociate_frame_builder.fragment_number(11);

  disassociate_frame_builder.reason_code(ReasonCode::STALeavingBSS);

  let disassociate_frame = disassociate_frame_builder.build();

  assert_eq!(
    disassociate_frame.version(),
    FrameVersion::Standard,
    "version"
  );
  assert_eq!(
    disassociate_frame.subtype(),
    FrameSubtype::Management(ManagementSubtype::Beacon),
    "subtype"
  );
  assert_eq!(
    disassociate_frame.ds_status(),
    DSStatus::FromDSToSTA,
    "ds_status"
  );

  assert_eq!(disassociate_frame.more_fragments(), false, "more_fragments");
  assert_eq!(disassociate_frame.retry(), true, "retry");
  assert_eq!(disassociate_frame.pwr_mgt(), true, "pwr_mgt");
  assert_eq!(disassociate_frame.more_data(), false, "more_disassociate");
  assert_eq!(disassociate_frame.protected(), false, "protected");
  assert_eq!(disassociate_frame.order(), false, "order");

  assert_eq!(
    disassociate_frame.source_address().expect("source_address"),
    "11:22:33:44:55:66".parse().unwrap(),
    "source_address"
  );

  assert_eq!(
    disassociate_frame
      .destination_address()
      .expect("destination_address"),
    "22:22:33:44:55:66".parse().unwrap(),
    "destination_address"
  );

  assert_eq!(
    disassociate_frame.bssid_address().expect("bssid_address"),
    "33:22:33:44:55:66".parse().unwrap(),
    "bssid_address"
  );

  assert_eq!(disassociate_frame.sequence_number(), 10);
  assert_eq!(disassociate_frame.fragment_number(), 11);

  assert_eq!(
    disassociate_frame.reason_code(),
    ReasonCode::STALeavingBSS,
    "reason_code"
  );
}
