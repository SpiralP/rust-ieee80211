mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AssociationResponseFrame {
    bytes: Bytes,
}

impl AssociationResponseFrame {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for AssociationResponseFrame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }
}
impl FragmentSequenceTrait for AssociationResponseFrame {}
impl ManagementFrameTrait for AssociationResponseFrame {}
impl AssociationResponseFixedParametersTrait for AssociationResponseFrame {}
impl TaggedParametersTrait for AssociationResponseFrame {
    const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
