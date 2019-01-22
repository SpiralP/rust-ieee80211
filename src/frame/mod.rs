pub mod ethernet;
pub mod ieee802_11;
pub mod ip;

pub use eui48::MacAddress;

pub trait FrameTrait<'a> {
  fn bytes(&self) -> &'a [u8];
}
