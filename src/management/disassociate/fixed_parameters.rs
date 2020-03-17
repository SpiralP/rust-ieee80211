use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait DisassociateFixedParametersTrait: FrameTrait {
    const FIXED_PARAMETERS_START: usize = 24;
    const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 6;

    fn reason_code(&self) -> ReasonCode {
        ReasonCode::from_u16(LittleEndian::read_u16(
            &self.bytes()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 2)],
        ))
    }
}

pub trait DisassociateFixedParametersBuilderTrait: FrameBuilderTrait {
    const FIXED_PARAMETERS_START: usize = 24;
    const FIXED_PARAMETERS_END: usize = Self::FIXED_PARAMETERS_START + 6;

    fn reason_code(&mut self, reason_code: ReasonCode) {
        LittleEndian::write_u16(
            &mut self.bytes_mut()[Self::FIXED_PARAMETERS_START..(Self::FIXED_PARAMETERS_START + 2)],
            reason_code.into_u16(),
        )
    }
}
