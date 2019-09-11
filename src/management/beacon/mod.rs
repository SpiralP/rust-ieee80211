mod fixed_parameters;

pub use self::fixed_parameters::*;
use super::*;

pub struct BeaconFrame {
  bytes: Bytes,
}

impl BeaconFrame {
  pub fn new(bytes: Bytes) -> Self {
    Self { bytes }
  }
}

impl FrameTrait for BeaconFrame {
  fn bytes(&self) -> Bytes {
    self.bytes.clone()
  }
}
impl FragmentSequenceTrait for BeaconFrame {}
impl ManagementFrameTrait for BeaconFrame {}
impl BeaconFixedParametersTrait for BeaconFrame {}
impl TaggedParametersTrait for BeaconFrame {}
