use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait AssociationRequestFixedParametersTrait<'a>: FrameTrait<'a> {
  const FIXED_PARAMETERS_START: usize = 24;
  const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 4;

  fn capabilities_info(&self) -> CapabilitiesInfo {
    CapabilitiesInfo::from_bytes(
      &self.bytes()[(Self::FIXED_PARAMETERS_START)..(Self::FIXED_PARAMETERS_START + 2)],
    )
  }

  fn listen_interval(&self) -> u16 {
    LittleEndian::read_u16(
      &self.bytes()[(Self::FIXED_PARAMETERS_START + 2)..(Self::FIXED_PARAMETERS_START + 4)],
    )
  }
}
