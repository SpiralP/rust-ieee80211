mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AssociationRequestFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> AssociationRequestFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for AssociationRequestFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for AssociationRequestFrame<'_> {}
impl ManagementFrameTrait for AssociationRequestFrame<'_> {}
impl AssociationRequestFixedParametersTrait for AssociationRequestFrame<'_> {}
impl TaggedParametersTrait for AssociationRequestFrame<'_> {
    const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}

use std::fmt;
impl fmt::Display for AssociationRequestFrame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssociationRequest")?;

        if let Some(transmitter_address) = self.transmitter_address() {
            write!(f, " tx: {}", transmitter_address)?;
        }

        write!(f, " rx: {}", self.receiver_address())?;

        Ok(())
    }
}
