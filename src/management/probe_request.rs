use super::*;

pub struct ProbeRequestFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> ProbeRequestFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for ProbeRequestFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for ProbeRequestFrame<'_> {}
impl ManagementFrameTrait for ProbeRequestFrame<'_> {}
impl TaggedParametersTrait for ProbeRequestFrame<'_> {
    const TAGGED_PARAMETERS_START: usize = 24;
}
