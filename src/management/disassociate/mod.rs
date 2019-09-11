mod builder;
mod fixed_parameters;

pub use self::{builder::*, fixed_parameters::*};
use super::*;

pub struct DisassociateFrame {
  bytes: Bytes,
}

impl DisassociateFrame {
  pub fn new(bytes: Bytes) -> Self {
    Self { bytes }
  }
}
impl FrameTrait for DisassociateFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }
}
impl FragmentSequenceTrait for DisassociateFrame {}
impl ManagementFrameTrait for DisassociateFrame {}
impl DisassociateFixedParametersTrait for DisassociateFrame {}
