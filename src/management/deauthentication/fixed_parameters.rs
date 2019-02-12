use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait DeauthenticationFixedParametersTrait<'a>: FrameTrait<'a> {
  const FIXED_PARAMETERS_START: usize = 24;
  const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 6;

  fn reason_code(&self) -> ReasonCode {
    ReasonCode::from_u16(LittleEndian::read_u16(
      &self.bytes()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 2)],
    ))
  }
}
