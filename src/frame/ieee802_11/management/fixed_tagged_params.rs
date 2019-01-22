use super::*;

pub trait FixedTaggedParamsTrait<'a>: IEEE802_11FrameTrait<'a> {
  fn ssid(&self) -> Option<Vec<u8>> {
    self.tagged_parameters().ssid()
  }

  fn timestamp(&self) -> u64 {
    // let timestamp: u64 = bytes.read_u64::<LE>().unwrap();
    // &self.bytes()[24..32];
    unimplemented!()
  }

  /// in seconds
  fn beacon_interval(&self) -> f64 {
    // self.bytes()[32..34]
    // let beacon_interval = f64::from(bytes.read_u16::<LE>().unwrap()) * 0.001_024f64;

    unimplemented!()
  }

  fn capabilities_info(&self) -> CapabilitiesInfo {
    // 34..36
    unimplemented!()
  }

  fn tagged_parameters(&self) -> TaggedParameters {
    let mut tagged_parameters = TaggedParameters::new();

    let mut i = 36;

    let bytes = self.bytes();
    let len = bytes.len();

    while i < len {
      let tag_number: u8 = bytes[i];

      i += 1;
      let tag_length: usize = bytes[i] as usize;

      i += 1;
      let data = &bytes[i..(i + tag_length)];
      i += tag_length;

      tagged_parameters.add(tag_number, data);
    }

    tagged_parameters
  }
}
