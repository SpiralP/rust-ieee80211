use super::*;

pub trait FragmentSequenceTrait<'a>: FrameTrait<'a> {
  const FRAGMENT_SEQUENCE_START: usize = 22;

  /// Fragment Number
  fn fragment_number(&self) -> u8 {
    (self.bytes()[Self::FRAGMENT_SEQUENCE_START] & 0b0000_1111) as u8
  }

  /// Sequence Number
  fn sequence_number(&self) -> u16 {
    let left = u16::from(self.bytes()[Self::FRAGMENT_SEQUENCE_START + 1]);
    let right = u16::from(self.bytes()[Self::FRAGMENT_SEQUENCE_START]);
    (right | (left << 8)) >> 4
  }
}
