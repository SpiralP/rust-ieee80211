mod builder;

pub use self::builder::*;
use super::*;
use byteorder::{ByteOrder, LittleEndian};
use bytes::Bytes;

pub enum FrameLayer {
    Management(ManagementFrame),
    Control(ControlFrame),
    Data(DataFrame),
}

pub struct Frame {
    bytes: Bytes,
}
impl Frame {
    pub fn new<T: Into<Bytes>>(bytes: T) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    pub fn next_layer(&self) -> Option<FrameLayer> {
        match self.type_() {
            FrameType::Management => {
                Some(FrameLayer::Management(ManagementFrame::new(self.bytes())))
            }
            FrameType::Control => Some(FrameLayer::Control(ControlFrame::new(self.bytes()))),
            FrameType::Data => Some(FrameLayer::Data(DataFrame::new(self.bytes()))),
            _ => None,
        }
    }
}
impl FrameTrait for Frame {
    fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }
}

pub trait FrameTrait {
    fn bytes(&self) -> Bytes;

    fn version(&self) -> FrameVersion {
        FrameVersion::from_u8(self.bytes()[0] & 0b0000_0011)
    }

    /// Main IEEE 802.11 Frame Type
    fn type_(&self) -> FrameType {
        FrameType::from_u8((self.bytes()[0] & 0b0000_1100) >> 2)
    }

    /// IEEE 802.11 Frame Subtype
    fn subtype(&self) -> FrameSubtype {
        let subtype = (self.bytes()[0] & 0b1111_0000) >> 4;

        FrameSubtype::from_u8(self.type_(), subtype)
    }

    // flags

    /// to Distribution System
    fn to_ds(&self) -> bool {
        self.bytes()[1] & 0b0000_0001 != 0
    }

    /// from Distribution System
    #[allow(clippy::wrong_self_convention)]
    fn from_ds(&self) -> bool {
        self.bytes()[1] & 0b0000_0010 != 0
    }

    fn ds_status(&self) -> DSStatus {
        DSStatus::from_bools(self.from_ds(), self.to_ds())
    }

    /// 0: This is the last fragment
    /// 1: More fragments follow
    fn more_fragments(&self) -> bool {
        (self.bytes()[1] & 0b0000_0100) != 0
    }

    /// 0: Frame is not being retransmitted
    /// 1: Frame is being retransmitted
    fn retry(&self) -> bool {
        (self.bytes()[1] & 0b0000_1000) != 0
    }

    /// 0: STA will stay up
    /// 1: STA will go to sleep
    fn pwr_mgt(&self) -> bool {
        (self.bytes()[1] & 0b0001_0000) != 0
    }

    /// 0: No data buffered
    /// 1: Data is buffered for STA at AP
    fn more_data(&self) -> bool {
        (self.bytes()[1] & 0b0010_0000) != 0
    }

    /// 0: Data is not protected
    /// 1: Data is protected
    fn protected(&self) -> bool {
        (self.bytes()[1] & 0b0100_0000) != 0
    }

    /// 0: Not strictly ordered
    /// 1: Strictly ordered
    fn order(&self) -> bool {
        // TODO also used in QoSData/Management frames for if HT Control exists
        (self.bytes()[1] & 0b1000_0000) != 0
    }

    /// Duration or Association Identifier
    fn duration_or_id(&self) -> DurationID {
        if (self.bytes()[3] & 0b1000_0000) == 0 {
            let n = LittleEndian::read_u16(&self.bytes()[2..4]) & 0b0111_1111_1111_1111;
            DurationID::Duration(n)
        } else {
            let n = LittleEndian::read_u16(&self.bytes()[2..4]) & 0b0011_1111_1111_1111;
            // valid range 1-2007
            if n < 1 || n > 2007 {
                DurationID::Reserved(n)
            } else {
                DurationID::AssociationID(n)
            }
        }
    }

    // Addressing

    fn addr1(&self) -> MacAddress {
        MacAddress::from_bytes(&self.bytes()[4..10]).unwrap()
    }

    /// Receiver Address
    /// Who this packet is destined for wirelessly.
    /// Address 1
    fn receiver_address(&self) -> MacAddress {
        self.addr1()
    }

    /// Destination Address
    /// Who the packet is destined for.
    fn destination_address(&self) -> Option<MacAddress> {
        Some(self.receiver_address())
    }
}
