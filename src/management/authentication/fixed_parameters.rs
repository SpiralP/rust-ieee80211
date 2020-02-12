use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait AuthenticationFixedParametersTrait: FrameTrait {
  const FIXED_PARAMETERS_START: usize = 24;
  const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 6;

  fn authentication_algorithm(&self) -> AuthenticationAlgorithm {
    AuthenticationAlgorithm::from_u16(LittleEndian::read_u16(
      &self.bytes()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 2)],
    ))
  }

  fn authentication_seq(&self) -> u16 {
    LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 2)..(Self::FIXED_PARAMETERS_START + 4)],
    )
  }

  fn status_code(&self) -> StatusCode {
    StatusCode::from_u16(LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 4)..(Self::FIXED_PARAMETERS_START + 6)],
    ))
  }
}

#[derive(Debug, PartialEq)]
pub enum AuthenticationAlgorithm {
  OpenSystem,
  SharedKey,
  FastBSSTransition,
  SAE,
  FILSSharedKeyWithoutPFS,
  FILSSharedKeyWithPFS,
  FILSPublicKey,
  NetworkEAP, // 0x80
  Reserved(u16),
}
impl AuthenticationAlgorithm {
  pub fn from_u16(n: u16) -> Self {
    match n {
      0 => Self::OpenSystem,
      1 => Self::SharedKey,
      2 => Self::FastBSSTransition,
      3 => Self::SAE,
      4 => Self::FILSSharedKeyWithoutPFS,
      5 => Self::FILSSharedKeyWithPFS,
      6 => Self::FILSPublicKey,
      0x80 => Self::NetworkEAP, // 0x80
      other => Self::Reserved(other),
    }
  }
}
