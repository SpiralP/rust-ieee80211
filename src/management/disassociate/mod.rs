mod builder;
mod fixed_parameters;

pub use self::{builder::*, fixed_parameters::*};
use super::*;

pub struct DisassociateFrame<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> DisassociateFrame<'a> {
    pub fn new<T: Into<Cow<'a, [u8]>>>(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }
}
impl FrameTrait for DisassociateFrame<'_> {
    fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}
impl FragmentSequenceTrait for DisassociateFrame<'_> {}
impl ManagementFrameTrait for DisassociateFrame<'_> {}
impl DisassociateFixedParametersTrait for DisassociateFrame<'_> {}
