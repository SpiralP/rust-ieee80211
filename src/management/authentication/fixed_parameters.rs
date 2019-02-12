use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait AuthenticationFixedParametersTrait<'a>: FrameTrait<'a> {
  const FIXED_PARAMETERS_START: usize = 24;

  fn authentication_algorithm(&self) -> AuthenticationAlgorithm {
    // 0: Open System
    AuthenticationAlgorithm::from_u16(LittleEndian::read_u16(
      &self.bytes()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 2)],
    ))
  }

  fn authentication_seq(&self) -> u16 {
    // 0x0001
    LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 2)..(Self::FIXED_PARAMETERS_START + 4)],
    )
  }

  fn status_code(&self) -> StatusCode {
    // 0x0000 Successful
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
      0 => AuthenticationAlgorithm::OpenSystem,
      1 => AuthenticationAlgorithm::SharedKey,
      2 => AuthenticationAlgorithm::FastBSSTransition,
      3 => AuthenticationAlgorithm::SAE,
      4 => AuthenticationAlgorithm::FILSSharedKeyWithoutPFS,
      5 => AuthenticationAlgorithm::FILSSharedKeyWithPFS,
      6 => AuthenticationAlgorithm::FILSPublicKey,
      0x80 => AuthenticationAlgorithm::NetworkEAP, // 0x80
      other => AuthenticationAlgorithm::Reserved(other),
    }
  }
}
