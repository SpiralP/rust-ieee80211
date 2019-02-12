mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct BeaconFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> BeaconFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}

impl<'a> FrameTrait<'a> for BeaconFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for BeaconFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for BeaconFrame<'a> {}
impl<'a> BeaconFixedParametersTrait<'a> for BeaconFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for BeaconFrame<'a> {}
