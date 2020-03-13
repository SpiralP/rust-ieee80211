mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct BeaconFrame<'a> {
    bytes: &'a [u8],
}

impl<'a> BeaconFrame<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl FrameTrait for BeaconFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes
    }
}
impl FragmentSequenceTrait for BeaconFrame<'_> {}
impl ManagementFrameTrait for BeaconFrame<'_> {}
impl BeaconFixedParametersTrait for BeaconFrame<'_> {}
impl TaggedParametersTrait for BeaconFrame<'_> {}
