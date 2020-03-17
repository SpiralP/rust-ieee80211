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
impl FrameTrait for AuthenticationFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for AuthenticationFrame<'_> {}
impl ManagementFrameTrait for AuthenticationFrame<'_> {}
impl AuthenticationFixedParametersTrait for AuthenticationFrame<'_> {}
impl TaggedParametersTrait for AuthenticationFrame<'_> {
    const TAGGED_PARAMETERS_START: usize = Self::FIXED_PARAMETERS_END;
}
