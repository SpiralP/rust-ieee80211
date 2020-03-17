use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait AssociationResponseFixedParametersTrait: FrameTrait {
    const FIXED_PARAMETERS_START: usize = 24;
    const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 6;

    fn capabilities_info(&self) -> CapabilitiesInfo {
        CapabilitiesInfo::from_bytes(
            &self.bytes()[(Self::FIXED_PARAMETERS_START)..(Self::FIXED_PARAMETERS_START + 2)],
        )
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(LittleEndian::read_u16(
            &self.bytes()[(Self::FIXED_PARAMETERS_START + 2)..(Self::FIXED_PARAMETERS_START + 4)],
        ))
    }

    fn association_id(&self) -> u16 {
        // TODO skip first 2 bits?
        LittleEndian::read_u16(
            &self.bytes()[(Self::FIXED_PARAMETERS_START + 4)..(Self::FIXED_PARAMETERS_START + 6)],
        ) & 0b0011_1111_1111_1111
    }
}
