use super::ip::{IPv4Frame, IPv6Frame};
use super::MacAddress;
use byteorder::{NetworkEndian, ReadBytesExt};

pub struct EthernetFrame<'a> {
  bytes: &'a [u8],
}
impl<'a> From<&'a [u8]> for EthernetFrame<'a> {
  fn from(bytes: &'a [u8]) -> EthernetFrame<'a> {
    EthernetFrame { bytes }
  }
}
impl<'a> EthernetFrame<'a> {
  pub fn destination(&self) -> MacAddress {
    MacAddress::from(&self.bytes[0..6])
  }

  pub fn source(&self) -> MacAddress {
    MacAddress::from(&self.bytes[6..12])
  }

  pub fn type_(&self) -> EthernetType {
    match (&self.bytes[12..14]).read_u16::<NetworkEndian>().unwrap() {
      0x0800 => EthernetType::IPv4,
      0x86DD => EthernetType::IPv6,
      _ => unreachable!(),
    }
  }

  pub fn next_layer(&self) -> EthernetLayer {
    match self.type_() {
      EthernetType::IPv4 => EthernetLayer::IPv4(IPv4Frame::from(&self.bytes[14..])),
      EthernetType::IPv6 => EthernetLayer::IPv6(IPv6Frame::from(&self.bytes[14..])),
    }
  }
}

pub enum EthernetLayer<'a> {
  IPv4(IPv4Frame<'a>),
  IPv6(IPv6Frame<'a>),
}

#[derive(Debug)]
pub enum EthernetType {
  IPv4,
  IPv6,
}
