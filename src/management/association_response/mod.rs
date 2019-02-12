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
impl<'a> FrameTrait<'a> for AssociationResponseFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for AssociationResponseFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for AssociationResponseFrame<'a> {}
impl<'a> AssociationResponseFixedParametersTrait<'a> for AssociationResponseFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for AssociationResponseFrame<'a> {
  const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
