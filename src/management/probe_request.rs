use super::*;

pub struct ProbeRequestFrame {
    bytes: Bytes,
}

impl ProbeRequestFrame {
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }
}
impl FrameTrait for ProbeRequestFrame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }
}
impl FragmentSequenceTrait for ProbeRequestFrame {}
impl ManagementFrameTrait for ProbeRequestFrame {}
impl TaggedParametersTrait for ProbeRequestFrame {
    const TAGGED_PARAMETERS_START: usize = 24;
}
