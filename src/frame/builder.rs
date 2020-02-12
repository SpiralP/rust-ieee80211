use super::*;

const FRAME_SIZE: usize = 10;

#[derive(Default)]
pub struct FrameBuilder {
  bytes: [u8; FRAME_SIZE],
}
impl FrameBuilder {
  pub fn new() -> Self {
    let mut builder = Self {
      bytes: [0; FRAME_SIZE],
    };

    builder.version(FrameVersion::Standard);

    builder
  }

  pub fn build(&self) -> Frame {
    Frame::new(self.bytes().to_vec())
  }
}
impl FrameBuilderTrait for FrameBuilder {
  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }
}

pub trait FrameBuilderTrait {
  fn bytes(&self) -> &[u8];
  fn bytes_mut(&mut self) -> &mut [u8];

  fn version(&mut self, version: FrameVersion) {
    self.bytes_mut()[0] = (self.bytes()[0] & !0b0000_0011) | (0b0000_0011 & version.into_u8());
  }

  fn type_(&mut self, type_: FrameType) {
    self.bytes_mut()[0] = (self.bytes()[0] & !0b0000_1100) | (0b0000_1100 & (type_.into_u8() << 2));
  }

  fn subtype(&mut self, subtype: FrameSubtype) {
    self.bytes_mut()[0] =
      (self.bytes()[0] & !0b1111_0000) | (0b1111_0000 & (subtype.into_u8() << 4));
  }

  fn to_ds(&mut self, to_ds: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0000_0001) | (if to_ds { 0b0000_0001 } else { 0b0000_0000 });
  }

  fn from_ds(&mut self, from_ds: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0000_0010) | (if from_ds { 0b0000_0010 } else { 0b0000_0000 });
  }

  fn ds_status(&mut self, ds_status: DSStatus) {
    let (from_ds, to_ds) = ds_status.to_bools();
    self.from_ds(from_ds);
    self.to_ds(to_ds);
  }

  fn more_fragments(&mut self, more_fragments: bool) {
    self.bytes_mut()[1] = (self.bytes()[1] & !0b0000_0100)
      | (if more_fragments {
        0b0000_0100
      } else {
        0b0000_0000
      });
  }

  fn retry(&mut self, retry: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0000_1000) | (if retry { 0b0000_1000 } else { 0b0000_0000 });
  }

  fn pwr_mgt(&mut self, pwr_mgt: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0001_0000) | (if pwr_mgt { 0b0001_0000 } else { 0b0000_0000 });
  }

  fn more_data(&mut self, more_data: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0010_0000) | (if more_data { 0b0010_0000 } else { 0b0000_0000 });
  }

  fn protected(&mut self, protected: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b0100_0000) | (if protected { 0b0100_0000 } else { 0b0000_0000 });
  }

  fn order(&mut self, order: bool) {
    self.bytes_mut()[1] =
      (self.bytes()[1] & !0b1000_0000) | (if order { 0b1000_0000 } else { 0b0000_0000 });
  }

  fn duration_or_id(&mut self, _duration_or_id: DurationID) {
    unimplemented!()
  }

  fn addr1(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[4..10].copy_from_slice(mac_address.as_bytes());
  }

  /// Receiver Address
  /// Who this packet is destined for wirelessly.
  /// Address 1
  fn receiver_address(&mut self, mac_address: MacAddress) {
    self.addr1(mac_address)
  }

  /// Destination Address
  /// Who the packet is destined for.
  fn destination_address(&mut self, mac_address: MacAddress) {
    self.receiver_address(mac_address)
  }
}

#[test]
fn test_frame_builder() {
  let mut frame_builder = FrameBuilder::new();

  frame_builder.version(FrameVersion::Standard);
  frame_builder.type_(FrameType::Data);
  frame_builder.subtype(FrameSubtype::Data(DataSubtype::Null));
  frame_builder.ds_status(DSStatus::WDSOrMesh);

  frame_builder.more_fragments(true);
  frame_builder.retry(true);
  frame_builder.pwr_mgt(true);
  frame_builder.more_data(true);
  frame_builder.protected(true);
  frame_builder.order(true);

  frame_builder.receiver_address("11:22:33:44:55:66".parse().unwrap());

  let frame = frame_builder.build();

  assert_eq!(frame.version(), FrameVersion::Standard, "version");
  assert_eq!(frame.type_(), FrameType::Data, "type_");
  assert_eq!(
    frame.subtype(),
    FrameSubtype::Data(DataSubtype::Null),
    "subtype"
  );
  assert_eq!(frame.ds_status(), DSStatus::WDSOrMesh, "ds_status");

  assert_eq!(frame.more_fragments(), true, "more_fragments");
  assert_eq!(frame.retry(), true, "retry");
  assert_eq!(frame.pwr_mgt(), true, "pwr_mgt");
  assert_eq!(frame.more_data(), true, "more_data");
  assert_eq!(frame.protected(), true, "protected");
  assert_eq!(frame.order(), true, "order");

  assert_eq!(
    frame.receiver_address(),
    "11:22:33:44:55:66".parse().unwrap(),
    "receiver_address"
  );
}
