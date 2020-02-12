use super::*;

// TODO dynamic size
const DATA_FRAME_SIZE: usize = DataFrameBuilder::FRAGMENT_SEQUENCE_START + 2;

#[derive(Default)]
pub struct DataFrameBuilder {
  bytes: Vec<u8>,
}
impl DataFrameBuilder {
  #[must_use]
  pub fn new() -> Self {
    let mut builder = Self {
      bytes: vec![0; DATA_FRAME_SIZE],
    };

    builder.type_(FrameType::Data);

    builder
  }

  #[must_use]
  pub fn build(&self) -> DataFrame {
    DataFrame::new(self.bytes().to_vec())
  }

  pub fn next_layer(&mut self, data: &[u8]) {
    let mut index = Self::FRAGMENT_SEQUENCE_START + 2;

    let data_frame = self.build();

    if data_frame.protected() {
      index += 8; // skip TKIP/CCMP parameters
    }

    match data_frame.subtype() {
      FrameSubtype::Data(ref subtype) => match subtype {
        DataSubtype::QoSData => {
          index += 2; // skip Qos Control
        }
        DataSubtype::Data => {}

        // invalid type for holding data
        _ => return,
      },
      _ => unreachable!(),
    }

    self.bytes.resize(index + data.len(), 0);
    self.bytes_mut()[index..].copy_from_slice(data);
  }
}
impl FrameBuilderTrait for DataFrameBuilder {
  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn bytes_mut(&mut self) -> &mut [u8] {
    &mut self.bytes
  }

  fn addr1(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[4..10].copy_from_slice(mac_address.as_bytes());
  }

  fn destination_address(&mut self, mac_address: MacAddress) {
    match self.build().ds_status() {
      // TODO
      DSStatus::FromDSToSTA => self.addr1(mac_address),
      DSStatus::FromSTAToDS | DSStatus::WDSOrMesh => self.addr3(mac_address),
      // fall back to receiver
      _ => self.receiver_address(mac_address),
    };
  }
}

impl FragmentSequenceBuilderTrait for DataFrameBuilder {}
impl DataFrameBuilderTrait for DataFrameBuilder {
  fn transmitter_address(&mut self, mac_address: MacAddress) {
    self.addr2(mac_address)
  }

  fn source_address(&mut self, mac_address: MacAddress) {
    match self.build().ds_status() {
      DSStatus::FromDSToSTA => self.addr3(mac_address),
      DSStatus::FromSTAToDS => self.addr2(mac_address),
      DSStatus::WDSOrMesh => self.addr4(mac_address),
      // fall back to transmitter
      _ => self.transmitter_address(mac_address),
    };
  }

  fn bssid_address(&mut self, mac_address: MacAddress) {
    match self.build().ds_status() {
      DSStatus::FromDSToSTA => self.addr2(mac_address),
      DSStatus::FromSTAToDS => self.addr1(mac_address),
      DSStatus::NotLeavingDSOrADHOC => self.addr3(mac_address),
      _ => {}
    }
  }

  fn station_address(&mut self, mac_address: MacAddress) {
    match self.build().ds_status() {
      DSStatus::FromDSToSTA => self.addr1(mac_address),
      DSStatus::FromSTAToDS => self.addr2(mac_address),
      _ => {}
    }
  }
}

pub trait DataFrameBuilderTrait: FrameBuilderTrait {
  fn addr2(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[10..16].copy_from_slice(mac_address.as_bytes());
  }

  fn addr3(&mut self, mac_address: MacAddress) {
    self.bytes_mut()[16..22].copy_from_slice(mac_address.as_bytes());
  }

  fn addr4(&mut self, mac_address: MacAddress) {
    // only on Data Mesh types
    // after frag/seq numbers
    self.bytes_mut()[24..30].copy_from_slice(mac_address.as_bytes());
  }

  fn transmitter_address(&mut self, mac_address: MacAddress);
  fn source_address(&mut self, mac_address: MacAddress);

  fn bssid_address(&mut self, mac_address: MacAddress);

  fn station_address(&mut self, mac_address: MacAddress);
}

#[test]
fn test_data_frame_builder() {
  let mut data_frame_builder = DataFrameBuilder::new();

  data_frame_builder.version(FrameVersion::Standard);
  data_frame_builder.type_(FrameType::Data);
  data_frame_builder.subtype(FrameSubtype::Data(DataSubtype::QoSData));
  data_frame_builder.ds_status(DSStatus::FromDSToSTA);

  data_frame_builder.more_fragments(false);
  data_frame_builder.retry(true);
  data_frame_builder.pwr_mgt(true);
  data_frame_builder.more_data(false);
  data_frame_builder.protected(false);
  data_frame_builder.order(false);

  data_frame_builder.source_address("11:22:33:44:55:66".parse().unwrap());
  data_frame_builder.destination_address("22:22:33:44:55:66".parse().unwrap());
  data_frame_builder.bssid_address("33:22:33:44:55:66".parse().unwrap());

  data_frame_builder.sequence_number(10);
  data_frame_builder.fragment_number(11);

  data_frame_builder.next_layer(b"hello!!");

  let data_frame = data_frame_builder.build();

  assert_eq!(data_frame.version(), FrameVersion::Standard, "version");
  assert_eq!(data_frame.type_(), FrameType::Data, "type_");
  assert_eq!(
    data_frame.subtype(),
    FrameSubtype::Data(DataSubtype::QoSData),
    "subtype"
  );
  assert_eq!(data_frame.ds_status(), DSStatus::FromDSToSTA, "ds_status");

  assert_eq!(data_frame.more_fragments(), false, "more_fragments");
  assert_eq!(data_frame.retry(), true, "retry");
  assert_eq!(data_frame.pwr_mgt(), true, "pwr_mgt");
  assert_eq!(data_frame.more_data(), false, "more_data");
  assert_eq!(data_frame.protected(), false, "protected");
  assert_eq!(data_frame.order(), false, "order");

  assert_eq!(
    data_frame.source_address().unwrap(),
    "11:22:33:44:55:66".parse().unwrap(),
    "source_address"
  );

  assert_eq!(
    data_frame
      .destination_address()
      .expect("destination_address"),
    "22:22:33:44:55:66".parse().unwrap(),
    "destination_address"
  );

  assert_eq!(
    data_frame.bssid_address().expect("bssid_address"),
    "33:22:33:44:55:66".parse().unwrap(),
    "bssid_address"
  );

  assert_eq!(data_frame.sequence_number(), 10);
  assert_eq!(data_frame.fragment_number(), 11);

  assert_eq!(&data_frame.next_layer().unwrap()[..], b"hello!!");
}
