use super::*;
use byteorder::{ByteOrder, LittleEndian};
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
      1 => TagName::SupportedRates,
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

  pub fn supported_rates(&self) -> Option<Vec<f64>> {
    self
      .tags
      .get(&TagName::SupportedRates)
      .map(|supported_rates| {
        let mut rates = Vec::new();
        for rate in supported_rates {
          let n = match rate {
            0x82 => 1.0,
            0x84 => 2.0,
            0x8b => 5.5,
            0x8c => 6.0, // B
            0x12 => 9.0,
            0x96 => 11.0,
            0x98 => 12.0, // B
            0x24 => 18.0,
            0xa4 => 18.0, // B
            0x30 => 24.0,
            0xb0 => 24.0, // B
            0x48 => 36.0,
            0x60 => 48.0,
            0x6c => 54.0,
            _ => 0.0,
          };

          if n != 0.0 {
            rates.push(n);
          }
        }

        rates
      })
  }

  pub fn channel(&self) -> Option<u8> {
    self
      .tags
      .get(&TagName::DSParameter)
      .map(|bytes| bytes[0])
      // 5GHz
      .or_else(|| self.tags.get(&TagName::HTInformation).map(|bytes| bytes[0]))
  }

  pub fn rsn(&self) -> Option<RSN> {
    self.tags.get(&TagName::RSNInformation).map(|bytes| {
      let mut rsn = RSN::default();

      let mut i = 0;
      let len = bytes.len();

      let version = LittleEndian::read_u16(&bytes[i..(i + 2)]);
      rsn.version = version;
      i += 2;
      if i >= len {
        return rsn;
      }

      if version != 1 {
        unimplemented!("RSN version is {} != 1", version);
      }

      let group_cipher_suite = RSN::read_cipher_suite(&bytes[i..(i + 4)]);
      rsn.group_cipher_suite = Some(group_cipher_suite);
      i += 4;
      if i >= len {
        return rsn;
      }

      let pairwise_cipher_suite_count = LittleEndian::read_u16(&bytes[i..(i + 2)]);
      i += 2;
      if i >= len {
        return rsn;
      }

      for _ in 0..pairwise_cipher_suite_count {
        let pairwise_cipher_suite = RSN::read_cipher_suite(&bytes[i..(i + 4)]);
        rsn.pairwise_cipher_suites.push(pairwise_cipher_suite);
        i += 4;
        if i >= len {
          return rsn;
        }
      }

      let akm_suite_count = LittleEndian::read_u16(&bytes[i..(i + 2)]);
      i += 2;
      if i >= len {
        return rsn;
      }

      for _ in 0..akm_suite_count {
        let akm_suite = RSN::read_akm_suite(&bytes[i..(i + 4)]);
        rsn.akm_suites.push(akm_suite);
        i += 4;
        if i >= len {
          return rsn;
        }
      }

      rsn
    })
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct RSN {
  pub version: u16,
  pub group_cipher_suite: Option<CipherSuite>,
  pub pairwise_cipher_suites: Vec<CipherSuite>,
  pub akm_suites: Vec<AKMSuite>,
}

impl RSN {
  fn read_suite_oui_and_type(bytes: &[u8]) -> ([u8; 3], u8) {
    let mut suite_oui = [0; 3];
    suite_oui.clone_from_slice(&bytes[0..3]);
    let suite_type = bytes[3];
    (suite_oui, suite_type)
  }

  fn read_cipher_suite(bytes: &[u8]) -> CipherSuite {
    let (_oui, type_) = RSN::read_suite_oui_and_type(&bytes);

    debug_assert_eq!(
      _oui,
      [0x00, 0x0f, 0xac],
      "read_cipher_suite oui not IEEE802.11"
    );

    CipherSuite::from(type_)
  }

  fn read_akm_suite(bytes: &[u8]) -> AKMSuite {
    let (_oui, type_) = RSN::read_suite_oui_and_type(&bytes);

    debug_assert_eq!(
      _oui,
      [0x00, 0x0f, 0xac],
      "read_akm_suite oui not IEEE802.11"
    );

    AKMSuite::from(type_)
  }
}

#[derive(Debug, PartialEq)]
pub enum CipherSuite {
  WEP40 = 1,
  TKIP = 2,
  CCMP = 4, // AES (CCM)
  WEP104 = 5,
}
impl CipherSuite {
  fn from(type_: u8) -> Self {
    match type_ {
      1 => CipherSuite::WEP40,
      2 => CipherSuite::TKIP,
      4 => CipherSuite::CCMP, // AES (CCM)
      5 => CipherSuite::WEP104,
      other => unimplemented!("Cipher Suite {}", other),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum AKMSuite {
  IEEE802_1X = 1,
  PSK = 2,
  FTOver802_1X = 3,
}
impl AKMSuite {
  fn from(type_: u8) -> Self {
    match type_ {
      1 => AKMSuite::IEEE802_1X,
      2 => AKMSuite::PSK,
      3 => AKMSuite::FTOver802_1X,
      other => unimplemented!("AKM Suite {}", other),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum TagName {
  Other(u8),
  SSID,
  SupportedRates,
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

// ch freq
// 36	5180
// 40	5200
// 44	5220
// 48	5240
// 52	5260
// 56	5280
// 60	5300
// 64	5320
// 100	5500
// 104	5520
// 108	5540
// 112	5560
// 116	5580
// 120	5600
// 124	5620
// 128	5640
// 132	5660
// 136	5680
// 140	5700
// 149	5745
// 153	5765
// 157	5785
// 161	5805
// 165	5825
