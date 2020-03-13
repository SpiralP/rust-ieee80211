mod builder;
mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct DeauthenticationFrame<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> DeauthenticationFrame<'a> {
    pub fn new<T: Into<Cow<'a, [u8]>>>(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}
impl FrameTrait for DeauthenticationFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}
impl FragmentSequenceTrait for DeauthenticationFrame<'_> {}
impl ManagementFrameTrait for DeauthenticationFrame<'_> {}
impl DeauthenticationFixedParametersTrait for DeauthenticationFrame<'_> {}
