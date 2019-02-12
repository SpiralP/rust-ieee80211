use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait AuthenticationFixedParametersTrait<'a>: FrameTrait<'a> {
  const FIXED_PARAMETERS_START: usize = 24;

  fn authentication_algorithm(&self) -> AuthenticationAlgorithm {
    // 0: Open System
    AuthenticationAlgorithm::Something(LittleEndian::read_u16(
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
    StatusCode::Something(LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 4)..(Self::FIXED_PARAMETERS_START + 6)],
    ))
  }
}

#[derive(Debug, PartialEq)]
pub enum AuthenticationAlgorithm {
  Something(u16),
}

#[derive(Debug, PartialEq)]
pub enum StatusCode {
  Something(u16),
}
