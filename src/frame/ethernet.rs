use super::ip::{IPv4Frame, IPv6Frame};
use byteorder::{NetworkEndian, ReadBytesExt};
use eui48::MacAddress;

pub struct EthernetFrame<'a> {
  bytes: &'a [u8],
}

impl<'a> EthernetFrame<'a> {
  pub fn new(bytes: &'a [u8]) -> EthernetFrame<'a> {
    EthernetFrame { bytes }
  }

  pub fn destination(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[0..6]).unwrap()
  }

  pub fn source(&self) -> MacAddress {
    MacAddress::from_bytes(&self.bytes[6..12]).unwrap()
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
      EthernetType::IPv4 => EthernetLayer::IPv4(IPv4Frame::new(&self.bytes[14..])),
      EthernetType::IPv6 => EthernetLayer::IPv6(IPv6Frame::new(&self.bytes[14..])),
    }
  }
}

pub enum EthernetLayer<'a> {
  IPv4(IPv4Frame<'a>),
  IPv6(IPv6Frame<'a>),
}

#[derive(Debug, PartialEq)]
pub enum EthernetType {
  IPv4,
  IPv6,
}

#[cfg(test)]
mod tests {
  use crate::ethernet::*;

  const ETHERNET_IPV4_TCP_ACK_PACKET: [u8; 74] = [
    0x00, 0x00, 0xc0, 0x9f, 0xa0, 0x97, 0x00, 0xa0, 0xcc, 0x3b, 0xbf, 0xfa, 0x08, 0x00, 0x45, 0x10,
    0x00, 0x3c, 0x46, 0x3c, 0x40, 0x00, 0x40, 0x06, 0x73, 0x1c, 0xc0, 0xa8, 0x00, 0x02, 0xc0, 0xa8,
    0x00, 0x01, 0x06, 0x0e, 0x00, 0x17, 0x99, 0xc5, 0xa0, 0xec, 0x00, 0x00, 0x00, 0x00, 0xa0, 0x02,
    0x7d, 0x78, 0xe0, 0xa3, 0x00, 0x00, 0x02, 0x04, 0x05, 0xb4, 0x04, 0x02, 0x08, 0x0a, 0x00, 0x9c,
    0x27, 0x24, 0x00, 0x00, 0x00, 0x00, 0x01, 0x03, 0x03, 0x00,
  ];

  #[test]
  fn test_ethernet_ipv4_tcp_ack_packet() {
    let ethernet_frame = EthernetFrame::new(&ETHERNET_IPV4_TCP_ACK_PACKET[..]);

    assert_eq!(
      ethernet_frame.destination(),
      "00:00:c0:9f:a0:97".parse().unwrap()
    );
    assert_eq!(
      ethernet_frame.source(),
      "00:a0:cc:3b:bf:fa".parse().unwrap()
    );

    assert_eq!(ethernet_frame.type_(), EthernetType::IPv4);

    match ethernet_frame.next_layer() {
      EthernetLayer::IPv4(_ipv4_frame) => {}
      _ => panic!("wrong layer type"),
    }
  }
}
