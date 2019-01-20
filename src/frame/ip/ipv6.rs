use super::IPVersion;

pub struct IPv6Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> From<&'a [u8]> for IPv6Frame<'a> {
  fn from(bytes: &'a [u8]) -> IPv6Frame<'a> {
    IPv6Frame { bytes }
  }
}

impl<'a> IPv6Frame<'a> {
  pub fn version(&self) -> IPVersion {
    match self.bytes[0] >> 4 {
      4 => IPVersion::IPv4,
      6 => IPVersion::IPv6,
      _ => panic!("other IPv6 version"),
    }
  }
}
