use super::*;
use byteorder::{ByteOrder, LittleEndian};

pub trait FragmentSequenceTrait<'a>: FrameTrait<'a> {
  const FRAGMENT_SEQUENCE_START: usize = 22;
  const FRAGMENT_SEQUENCE_END: usize = Self::FRAGMENT_SEQUENCE_START + 2;

  /// Fragment Number
  fn fragment_number(&self) -> u8 {
    (self.bytes()[Self::FRAGMENT_SEQUENCE_START] & 0b0000_1111) as u8
  }

  /// Sequence Number
  fn sequence_number(&self) -> u16 {
    LittleEndian::read_u16(
      &self.bytes()[Self::FRAGMENT_SEQUENCE_START..(Self::FRAGMENT_SEQUENCE_START + 2)],
    ) >> 4
  }
}

pub trait FragmentSequenceBuilderTrait: FrameBuilderTrait {
  const FRAGMENT_SEQUENCE_START: usize = 22;
  const FRAGMENT_SEQUENCE_END: usize = Self::FRAGMENT_SEQUENCE_START + 2;

  /// Fragment Number
  fn fragment_number(&mut self, fragment_number: u8) {
    self.bytes_mut()[Self::FRAGMENT_SEQUENCE_START] = (self.bytes()[Self::FRAGMENT_SEQUENCE_START]
      & !0b0000_1111)
      | (0b0000_1111 & fragment_number);
  }

  /// Sequence Number
  fn sequence_number(&mut self, sequence_number: u16) {
    let old = LittleEndian::read_u16(
      &self.bytes()[Self::FRAGMENT_SEQUENCE_START..(Self::FRAGMENT_SEQUENCE_START + 2)],
    ) & !0b1111_1111_1111_0000;

    LittleEndian::write_u16(
      &mut self.bytes_mut()[Self::FRAGMENT_SEQUENCE_START..(Self::FRAGMENT_SEQUENCE_START + 2)],
      old | (sequence_number << 4),
    );
  }
}
