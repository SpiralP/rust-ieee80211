use super::*;

pub struct ProbeResponseFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> ProbeResponseFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for ProbeResponseFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for ProbeResponseFrame<'_> {}
impl ManagementFrameTrait for ProbeResponseFrame<'_> {}
impl BeaconFixedParametersTrait for ProbeResponseFrame<'_> {}
impl TaggedParametersTrait for ProbeResponseFrame<'_> {
    // TODO: Check that this is correct
    const TAGGED_PARAMETERS_START: usize = 36;
}
