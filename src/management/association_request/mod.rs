mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AssociationRequestFrame {
    bytes: Bytes,
}

impl AssociationRequestFrame {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for AssociationRequestFrame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }
}
impl FragmentSequenceTrait for AssociationRequestFrame {}
impl ManagementFrameTrait for AssociationRequestFrame {}
impl AssociationRequestFixedParametersTrait for AssociationRequestFrame {}
impl TaggedParametersTrait for AssociationRequestFrame {
    const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}

use std::fmt;
impl fmt::Display for AssociationRequestFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssociationRequest")?;

        if let Some(transmitter_address) = self.transmitter_address() {
            write!(f, " tx: {}", transmitter_address)?;
        }

        write!(f, " rx: {}", self.receiver_address())?;

        Ok(())
    }
}
