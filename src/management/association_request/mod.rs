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
impl<'a> FrameTrait<'a> for AssociationRequestFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for AssociationRequestFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for AssociationRequestFrame<'a> {}
impl<'a> AssociationRequestFixedParametersTrait<'a> for AssociationRequestFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for AssociationRequestFrame<'a> {
  const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
