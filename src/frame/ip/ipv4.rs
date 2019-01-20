use super::IPVersion;

pub struct IPv4Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> From<&'a [u8]> for IPv4Frame<'a> {
  fn from(bytes: &'a [u8]) -> IPv4Frame<'a> {
    IPv4Frame { bytes }
  }
}

impl<'a> IPv4Frame<'a> {
  pub fn version(&self) -> IPVersion {
    match self.bytes[0] >> 4 {
      4 => IPVersion::IPv4,
      6 => IPVersion::IPv6,
      _ => panic!("other IPv4 version"),
    }
  }
}
