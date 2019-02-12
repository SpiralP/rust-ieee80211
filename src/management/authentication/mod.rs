mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AuthenticationFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> AuthenticationFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}
impl<'a> FrameTrait<'a> for AuthenticationFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> FragmentSequenceTrait<'a> for AuthenticationFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for AuthenticationFrame<'a> {}
impl<'a> AuthenticationFixedParametersTrait<'a> for AuthenticationFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for AuthenticationFrame<'a> {
  const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
