mod builder;
mod fixed_parameters;

pub use self::builder::*;
pub use self::fixed_parameters::*;
use super::*;

pub struct DeauthenticationFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> DeauthenticationFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}
impl<'a> FrameTrait<'a> for DeauthenticationFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for DeauthenticationFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for DeauthenticationFrame<'a> {}
impl<'a> DeauthenticationFixedParametersTrait<'a> for DeauthenticationFrame<'a> {}
