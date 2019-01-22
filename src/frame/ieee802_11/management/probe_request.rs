use super::*;

pub struct ProbeRequestFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> ProbeRequestFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> Self {
    Self { bytes }
  }
}
impl<'a> FrameTrait<'a> for ProbeRequestFrame<'a> {
  fn bytes(&self) -> &'a [u8] {
    self.bytes
  }
}
impl<'a> IEEE802_11FrameTrait<'a> for ProbeRequestFrame<'a> {}
impl<'a> FragmentSequenceTrait<'a> for ProbeRequestFrame<'a> {}
impl<'a> ManagementFrameTrait<'a> for ProbeRequestFrame<'a> {}
impl<'a> TaggedParametersTrait<'a> for ProbeRequestFrame<'a> {
  const TAGGED_PARAMETERS_START: usize = 24;
}
