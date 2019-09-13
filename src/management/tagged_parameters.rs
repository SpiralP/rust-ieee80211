use super::*;
use bytes::{ByteOrder, LittleEndian};
use std::collections::HashMap;

#[derive(Default)]
pub struct TaggedParameters {
  tags: HashMap<TagName, Vec<u8>>,
}

impl TaggedParameters {
  pub fn new() -> Self {
    Self {
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

  pub fn get_all(&self) -> &HashMap<TagName, Vec<u8>> {
    &self.tags
  }

  pub fn get_bytes(&self, tag_name: TagName) -> Option<Vec<u8>> {
    self.tags.get(&tag_name).cloned()
  }

  pub fn ssid(&self) -> Option<Vec<u8>> {
    self.get_bytes(TagName::SSID)
  }

  /// in Mbit/sec
  pub fn supported_rates(&self) -> Option<Vec<f64>> {
    self
      .tags
      .get(&TagName::SupportedRates)
      .map(|supported_rates| {
        let mut rates = Vec::new();
        for rate in supported_rates {
          // let is_basic = (rate & 0b1000_0000) != 0;
          let kbps = rate & 0b0111_1111;

          rates.push((f64::from(kbps) * 500.0) / 1000.0);
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

  pub fn rsn(&self) -> Option<RSNVersion> {
    self.tags.get(&TagName::RSNInformation).and_then(|bytes| {
      let mut i = 0;
      let len = bytes.len();

      if (i + 1) >= len {
        return None;
      }
      let version = LittleEndian::read_u16(&bytes[i..=(i + 1)]);
      Some(match version {
        1 => {
          i += 2;
          RSNVersion::Standard(make_std_rsn(&bytes[i..]))
        }
        other => RSNVersion::Reserved(other),
      })
    })
  }
}

fn make_std_rsn(bytes: &[u8]) -> RSN {
  let mut i = 0;
  let len = bytes.len();

  let mut rsn = RSN::default();

  if (i + 4) > len {
    return rsn;
  }
  let group_cipher_suite = RSN::read_cipher_suite(&bytes[i..(i + 4)]);
  rsn.group_cipher_suite = Some(group_cipher_suite);
  i += 4;

  if (i + 2) > len {
    return rsn;
  }
  let pairwise_cipher_suite_count = LittleEndian::read_u16(&bytes[i..(i + 2)]);
  i += 2;

  for _ in 0..pairwise_cipher_suite_count {
    if (i + 4) > len {
      return rsn;
    }
    let pairwise_cipher_suite = RSN::read_cipher_suite(&bytes[i..(i + 4)]);
    rsn.pairwise_cipher_suites.push(pairwise_cipher_suite);
    i += 4;
  }

  if (i + 2) > len {
    return rsn;
  }
  let akm_suite_count = LittleEndian::read_u16(&bytes[i..(i + 2)]);
  i += 2;

  for _ in 0..akm_suite_count {
    if (i + 4) > len {
      return rsn;
    }
    let akm_suite = RSN::read_akm_suite(&bytes[i..(i + 4)]);
    rsn.akm_suites.push(akm_suite);
    i += 4;
  }

  if (i + 2) > len {
    return rsn;
  }
  let b = LittleEndian::read_u16(&bytes[i..(i + 2)]);
  rsn.capabilities = Some(RSNCapabilities {
    pre_auth: (b & 0b0000_0000_0000_0001) != 0,
    pairwise: (b & 0b0000_0000_0000_0010) != 0,
    ptksa_replay_counter_value: ((b & 0b0000_0000_0000_1100) >> 2) as u8,
    gtksa_replay_counter_value: ((b & 0b0000_0000_0011_0000) >> 4) as u8,
    management_frame_protection_required: (b & 0b0000_0000_0100_0000) != 0,
    management_frame_protection_capable: (b & 0b0000_0000_1000_0000) != 0,
    joint_multi_band_rsna: (b & 0b0000_0001_0000_0000) != 0,
    peerkey: (b & 0b0000_0010_0000_0000) != 0,
  });

  rsn
}

#[derive(Debug, PartialEq)]
pub struct RSNCapabilities {
  /// 0: RSN Pre-Auth capabilities: Transmitter does not support pre-authentication
  pub pre_auth: bool,
  /// 0: RSN No Pairwise capabilities: Transmitter can support WEP default key 0 simultaneously with Pairwise key
  pub pairwise: bool,

  // {0x00, "1 replay counter per PTKSA/GTKSA/STAKeySA"},
  // {0x01, "2 replay counters per PTKSA/GTKSA/STAKeySA"},
  // {0x02, "4 replay counters per PTKSA/GTKSA/STAKeySA"},
  // {0x03, "16 replay counters per PTKSA/GTKSA/STAKeySA"},
  pub ptksa_replay_counter_value: u8,
  pub gtksa_replay_counter_value: u8,

  /// Management Frame Protection Required
  pub management_frame_protection_required: bool,

  /// Management Frame Protection Capable
  pub management_frame_protection_capable: bool,

  /// Joint Multi-band RSNA
  pub joint_multi_band_rsna: bool,

  /// PeerKey Enabled
  pub peerkey: bool,
}

#[derive(Debug, PartialEq)]
pub enum RSNVersion {
  Standard(RSN), // 1
  Reserved(u16),
}

#[derive(Debug, Default, PartialEq)]
pub struct RSN {
  pub group_cipher_suite: Option<CipherSuite>,
  pub pairwise_cipher_suites: Vec<CipherSuite>,
  pub akm_suites: Vec<AKMSuite>,
  pub capabilities: Option<RSNCapabilities>,
}

impl RSN {
  fn read_suite_oui_and_type(bytes: &[u8]) -> ([u8; 3], u8) {
    let mut suite_oui = [0; 3];
    suite_oui.clone_from_slice(&bytes[0..3]);
    let suite_type = bytes[3];
    (suite_oui, suite_type)
  }

  fn read_cipher_suite(bytes: &[u8]) -> CipherSuite {
    let (oui, type_) = Self::read_suite_oui_and_type(&bytes);

    CipherSuite::from(oui, type_)
  }

  fn read_akm_suite(bytes: &[u8]) -> AKMSuite {
    let (oui, type_) = Self::read_suite_oui_and_type(&bytes);

    AKMSuite::from(oui, type_)
  }
}

#[derive(Debug, PartialEq)]
pub enum CipherSuite {
  Standard(CipherSuiteType),
  Vendor([u8; 3], u8),
}
impl CipherSuite {
  fn from(oui: [u8; 3], type_: u8) -> Self {
    match oui {
      [0x00, 0x0f, 0xac] => Self::Standard(CipherSuiteType::from(type_)),
      other => Self::Vendor(other, type_),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum CipherSuiteType {
  UseGroupCipherSuite, // 0
  WEP40,               // 1
  TKIP,                // 2
  Reserved(u8),        // 3 Reserved
  CCMP,                // 4 // AES (CCM)
  WEP104,              // 5
  BIP,                 // 6
  GroupAddressedTrafficNotAllowed, // 7
                       // 8-255 Reserved
}
impl CipherSuiteType {
  fn from(type_: u8) -> Self {
    match type_ {
      1 => Self::WEP40,
      2 => Self::TKIP,
      4 => Self::CCMP,
      5 => Self::WEP104,
      6 => Self::BIP,
      7 => Self::GroupAddressedTrafficNotAllowed,
      other => Self::Reserved(other),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum AKMSuite {
  Standard(AKMSuiteType),
  Vendor([u8; 3], u8),
}
impl AKMSuite {
  fn from(oui: [u8; 3], type_: u8) -> Self {
    match oui {
      [0x00, 0x0f, 0xac] => Self::Standard(AKMSuiteType::from(type_)),
      other => Self::Vendor(other, type_),
    }
  }
}

/// Authentication and Key Management Suite
#[derive(Debug, PartialEq)]
pub enum AKMSuiteType {
  Reserved(u8), // 0 Reserved

  /// IEEE 802.1X with RSNA default
  IEEE802_1X, // 1
  /// Pre-Shared-Key
  PSK, // 2
  /// FT auth negotiated over IEEE 802.1X
  FTOver802_1X, // 3
  /// FT auth using PSK
  FTPSK, // 4

  /// IEEE 802.1X with SHA256 Key Derivation
  IEEE802_1XSHA, // 5
  /// PSK with SHA256 Key Derivation
  PSKSHA, // 6
  /// TPK Handshake
  TDLS, // 7
  /// SAE with SHA256
  SAE, // 8
  /// FT auth over SAE with SHA256
  FTOverSAE, // 9

             //  10-255 Reserved
}
impl AKMSuiteType {
  fn from(type_: u8) -> Self {
    match type_ {
      1 => Self::IEEE802_1X,
      2 => Self::PSK,
      3 => Self::FTOver802_1X,
      4 => Self::FTPSK,
      5 => Self::IEEE802_1XSHA,
      6 => Self::PSKSHA,
      7 => Self::TDLS,
      8 => Self::SAE,
      9 => Self::FTOverSAE,
      other => Self::Reserved(other),
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

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct OverflowError {
  details: String,
}

impl OverflowError {
  pub fn new(details: String) -> Self {
    Self { details }
  }
}
impl fmt::Display for OverflowError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OverflowError: {}", self.details)
  }
}

impl Error for OverflowError {
  fn description(&self) -> &str {
    &self.details
  }
}

pub trait TaggedParametersTrait: FrameTrait {
  // Tagged Parameters (36..) on Beacon
  const TAGGED_PARAMETERS_START: usize = 36;

  fn tagged_parameters(&self) -> Result<TaggedParameters, OverflowError> {
    let mut tagged_parameters = TaggedParameters::new();

    let mut i = Self::TAGGED_PARAMETERS_START;

    let bytes = self.bytes();
    let len = bytes.len();

    while i < len {
      let tag_number: u8 = bytes[i];

      i += 1;
      let tag_length: usize = bytes[i] as usize;

      i += 1;

      if (i + tag_length) > len {
        return Err(OverflowError::new(format!("{} > {}", i + tag_length, len)));
      }

      let data = &bytes[i..(i + tag_length)];
      i += tag_length;

      tagged_parameters.add(tag_number, data);
    }

    Ok(tagged_parameters)
  }

  fn ssid(&self) -> Option<Vec<u8>> {
    self.tagged_parameters().ok()?.ssid()
  }
}
