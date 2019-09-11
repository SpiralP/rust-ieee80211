mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct AuthenticationFrame {
  bytes: Bytes,
}

impl AuthenticationFrame {
  pub fn new(bytes: Bytes) -> Self {
    Self { bytes }
  }
}
impl FrameTrait for AuthenticationFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }
}
impl FragmentSequenceTrait for AuthenticationFrame {}
impl ManagementFrameTrait for AuthenticationFrame {}
impl AuthenticationFixedParametersTrait for AuthenticationFrame {}
impl TaggedParametersTrait for AuthenticationFrame {
  const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
