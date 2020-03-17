mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AssociationResponseFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> AssociationResponseFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for AssociationResponseFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for AssociationResponseFrame<'_> {}
impl ManagementFrameTrait for AssociationResponseFrame<'_> {}
impl AssociationResponseFixedParametersTrait for AssociationResponseFrame<'_> {}
impl TaggedParametersTrait for AssociationResponseFrame<'_> {
    const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
