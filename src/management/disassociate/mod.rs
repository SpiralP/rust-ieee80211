mod builder;
mod fixed_parameters;

pub use self::builder::*;
pub use self::fixed_parameters::*;
use super::*;

pub struct DisassociateFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> DisassociateFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}
impl<'a> FrameTrait<'a> for DisassociateFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for DisassociateFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for DisassociateFrame<'a> {}
impl<'a> DisassociateFixedParametersTrait<'a> for DisassociateFrame<'a> {}
