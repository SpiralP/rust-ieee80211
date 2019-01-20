#[derive(Default)]
pub struct MacAddress {
  bytes: [u8; 6],
}

impl From<[u8; 6]> for MacAddress {
  fn from(bytes: [u8; 6]) -> MacAddress {
    MacAddress { bytes }
  }
}

impl From<&[u8]> for MacAddress {
  fn from(slice: &[u8]) -> MacAddress {
    let mut bytes: [u8; 6] = [0; 6];
    bytes.copy_from_slice(slice);
    MacAddress { bytes }
  }
}

impl std::fmt::Display for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!(
      "{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}:{:0>2X}",
      self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3], self.bytes[4], self.bytes[5],
    ))?;
    Ok(())
  }
}

impl std::fmt::Debug for MacAddress {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    formatter.write_fmt(format_args!("\"{}\"", self))?;
    Ok(())
  }
}
