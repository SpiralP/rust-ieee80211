use super::*;

pub struct ProbeResponseFrame {
    bytes: Bytes,
}

impl ProbeResponseFrame {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for ProbeResponseFrame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }
}
impl FragmentSequenceTrait for ProbeResponseFrame {}
impl ManagementFrameTrait for ProbeResponseFrame {}
impl BeaconFixedParametersTrait for ProbeResponseFrame {}
impl TaggedParametersTrait for ProbeResponseFrame {}
