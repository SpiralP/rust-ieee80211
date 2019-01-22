use super::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct TaggedParameters {
  tags: HashMap<TagName, Vec<u8>>,
}

impl TaggedParameters {
  pub fn new() -> Self {
    TaggedParameters {
      tags: HashMap::new(),
    }
  }

  pub fn add(&mut self, tag_number: u8, tag_data: &[u8]) {
    let tag_name = match tag_number {
      0 => TagName::SSID,
      1 => TagName::Rates,
      3 => TagName::DSParameter,
      5 => TagName::TrafficIndicationMap,
      7 => TagName::CountryInformation,
      42 => TagName::ERPInformation,
      50 => TagName::ExtendedSupportedRates,
      48 => TagName::RSNInformation,
      11 => TagName::QBSSLoadElement,
      45 => TagName::HTCapabilities,
      61 => TagName::HTInformation,
      127 => TagName::ExtendedCapabilities,

      n => TagName::Other(n),
    };

    self.tags.insert(tag_name, tag_data.to_vec());
  }

  pub fn get_bytes(&self, tag_name: TagName) -> Option<Vec<u8>> {
    self.tags.get(&tag_name).cloned()
  }

  pub fn ssid(&self) -> Option<Vec<u8>> {
    self.get_bytes(TagName::SSID)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum TagName {
  Other(u8),
  SSID,
  Rates,
  DSParameter,
  TrafficIndicationMap,
  CountryInformation,
  ERPInformation,
  ExtendedSupportedRates,
  RSNInformation,
  QBSSLoadElement,
  HTCapabilities,
  HTInformation,
  ExtendedCapabilities,
}

pub trait TaggedParametersTrait<'a>: FrameTrait<'a> {
  // Tagged Parameters (36..) on Beacon
  const TAGGED_PARAMETERS_START: usize = 36;

  fn tagged_parameters(&self) -> TaggedParameters {
    let mut tagged_parameters = TaggedParameters::new();

    let mut i = Self::TAGGED_PARAMETERS_START;

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

  fn ssid(&self) -> Option<Vec<u8>> {
    self.tagged_parameters().ssid()
  }
}
