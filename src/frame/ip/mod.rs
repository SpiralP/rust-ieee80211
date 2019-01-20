mod ipv4;
mod ipv6;

pub use self::ipv4::IPv4Frame;
pub use self::ipv6::IPv6Frame;

#[derive(Debug)]
pub enum IPVersion {
  IPv4,
  IPv6,
}
