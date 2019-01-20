use super::IPVersion;

pub struct IPv6Frame<'a> {
  bytes: &'a [u8],
}

impl<'a> IPv6Frame<'a> {
  pub fn from_bytes(bytes: &'a [u8]) -> IPv6Frame<'a> {
    IPv6Frame { bytes }
  }

  pub fn version(&self) -> IPVersion {
    match self.bytes[0] >> 4 {
      4 => IPVersion::IPv4,
      6 => IPVersion::IPv6,
      _ => panic!("other IPv6 version"),
    }
  }
}
