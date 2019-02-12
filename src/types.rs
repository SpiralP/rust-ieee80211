#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DSStatus {
  /// Not leaving DS or network is operating in AD-HOC mode
  NotLeavingDSOrADHOC,
  /// Frame from DS to a STA via AP
  FromDSToSTA,
  /// Frame from STA to DS via an AP
  FromSTAToDS,
  /// WDS (AP to AP) or Mesh (MP to MP) Frame
  WDSOrMesh,
}
impl DSStatus {
  pub fn from_bools(from_ds: bool, to_ds: bool) -> Self {
    match (from_ds, to_ds) {
      // 00 Not leaving DS or network is operating in AD-HOC mode
      (false, false) => DSStatus::NotLeavingDSOrADHOC,
      // 01 Frame from STA to DS via an AP
      (false, true) => DSStatus::FromSTAToDS,
      // 10 Frame from DS to a STA via AP
      (true, false) => DSStatus::FromDSToSTA,
      // 11 WDS (AP to AP) or Mesh (MP to MP) Frame
      (true, true) => DSStatus::WDSOrMesh,
    }
  }

  /// returns from_ds, to_ds
  pub fn to_bools(self) -> (bool, bool) {
    match self {
      // 00 Not leaving DS or network is operating in AD-HOC mode
      DSStatus::NotLeavingDSOrADHOC => (false, false),
      // 01 Frame from STA to DS via an AP
      DSStatus::FromSTAToDS => (false, true),
      // 10 Frame from DS to a STA via AP
      DSStatus::FromDSToSTA => (true, false),
      // 11 WDS (AP to AP) or Mesh (MP to MP) Frame
      DSStatus::WDSOrMesh => (true, true),
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DurationID {
  /// Microseconds
  Duration(u16),
  /// Association Identifier (AID)
  /// valid range 1-2007
  AssociationID(u16),

  Reserved(u16),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrameVersion {
  Standard, // 0
  Reserved(u8),
}

impl FrameVersion {
  pub fn from_u8(n: u8) -> Self {
    match n {
      0 => FrameVersion::Standard,
      other => FrameVersion::Reserved(other),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      FrameVersion::Standard => 0,
      FrameVersion::Reserved(other) => other,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrameType {
  Management, // 0
  Control,    // 1
  Data,       // 2
  Reserved(u8),
}

impl FrameType {
  pub fn from_u8(n: u8) -> Self {
    match n {
      0 => FrameType::Management,
      1 => FrameType::Control,
      2 => FrameType::Data,
      other => FrameType::Reserved(other),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      FrameType::Management => 0,
      FrameType::Control => 1,
      FrameType::Data => 2,
      FrameType::Reserved(other) => other,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrameSubtype {
  Management(ManagementSubtype),
  Control(ControlSubtype),
  Data(DataSubtype),

  /// main type, subtype
  Reserved(u8, u8),
}

impl FrameSubtype {
  pub fn from_u8(type_: FrameType, subtype: u8) -> Self {
    match type_ {
      FrameType::Management => FrameSubtype::Management(ManagementSubtype::from_u8(subtype)),
      FrameType::Control => FrameSubtype::Control(ControlSubtype::from_u8(subtype)),
      FrameType::Data => FrameSubtype::Data(DataSubtype::from_u8(subtype)),
      FrameType::Reserved(type_) => FrameSubtype::Reserved(type_, subtype),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      FrameSubtype::Management(subtype) => subtype.to_u8(),
      FrameSubtype::Control(subtype) => subtype.to_u8(),
      FrameSubtype::Data(subtype) => subtype.to_u8(),
      FrameSubtype::Reserved(_type, subtype) => subtype,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ManagementSubtype {
  /// Association Request
  AssociationRequest, // 0
  /// Association Response
  AssociationResponse, // 1
  /// Reassociation Request
  ReassociationRequest, // 2
  /// Reassociation Response
  ReassociationResponse, // 3
  /// Probe Request
  ProbeRequest, // 4
  /// Probe Response
  ProbeResponse, // 5
  /// 6-7, 15 Reserved
  Reserved(u8), // 6-7
  /// Beacon
  Beacon, // 8
  /// Announcement Traffic Indication Message
  ATIM, // 9
  /// Disassociate
  Disassociate, // 10
  /// Authentication
  Authentication, // 11
  /// Deauthentication
  Deauthentication, // 12
  /// Action
  Action, // 13
  /// Action No Ack
  ActionNoAck, // 14
}

impl ManagementSubtype {
  pub fn from_u8(n: u8) -> Self {
    match n {
      0 => ManagementSubtype::AssociationRequest,
      1 => ManagementSubtype::AssociationResponse,
      2 => ManagementSubtype::ReassociationRequest,
      3 => ManagementSubtype::ReassociationResponse,
      4 => ManagementSubtype::ProbeRequest,
      5 => ManagementSubtype::ProbeResponse,
      // 6-7 Reserved
      8 => ManagementSubtype::Beacon,
      9 => ManagementSubtype::ATIM,
      10 => ManagementSubtype::Disassociate,
      11 => ManagementSubtype::Authentication,
      12 => ManagementSubtype::Deauthentication,
      13 => ManagementSubtype::Action,
      14 => ManagementSubtype::ActionNoAck,
      // 15 Reserved
      other => ManagementSubtype::Reserved(other),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      ManagementSubtype::AssociationRequest => 0,
      ManagementSubtype::AssociationResponse => 1,
      ManagementSubtype::ReassociationRequest => 2,
      ManagementSubtype::ReassociationResponse => 3,
      ManagementSubtype::ProbeRequest => 4,
      ManagementSubtype::ProbeResponse => 5,
      // 6-7 Reserved
      ManagementSubtype::Beacon => 8,
      ManagementSubtype::ATIM => 9,
      ManagementSubtype::Disassociate => 10,
      ManagementSubtype::Authentication => 11,
      ManagementSubtype::Deauthentication => 12,
      ManagementSubtype::Action => 13,
      ManagementSubtype::ActionNoAck => 14,
      // 15 Reserved
      ManagementSubtype::Reserved(other) => other,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ControlSubtype {
  /// 0-6 Reserved
  Reserved(u8), // 0-6
  /// Control Wrapper
  ControlWrapper, // 7
  /// Block Ack Request
  BlockAckRequest, // 8
  /// Block Ack
  BlockAck, // 9
  /// Power Save Poll
  PSPoll, // 10
  /// Request To Send
  RTS, // 11
  /// Clear To Send
  CTS, // 12
  /// Acknowledgement
  Ack, // 13
  /// Contention-Free-End
  CFEnd, // 14
  /// CF-End + CF-Ack
  CFEndCFAck, // 15
}

impl ControlSubtype {
  pub fn from_u8(n: u8) -> ControlSubtype {
    match n {
      // 0-6 Reserved
      7 => ControlSubtype::ControlWrapper,
      8 => ControlSubtype::BlockAckRequest,
      9 => ControlSubtype::BlockAck,
      10 => ControlSubtype::PSPoll,
      11 => ControlSubtype::RTS,
      12 => ControlSubtype::CTS,
      13 => ControlSubtype::Ack,
      14 => ControlSubtype::CFEnd,
      15 => ControlSubtype::CFEndCFAck,
      other => ControlSubtype::Reserved(other),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      // 0-6 Reserved
      ControlSubtype::ControlWrapper => 7,
      ControlSubtype::BlockAckRequest => 8,
      ControlSubtype::BlockAck => 9,
      ControlSubtype::PSPoll => 10,
      ControlSubtype::RTS => 11,
      ControlSubtype::CTS => 12,
      ControlSubtype::Ack => 13,
      ControlSubtype::CFEnd => 14,
      ControlSubtype::CFEndCFAck => 15,
      ControlSubtype::Reserved(other) => other,
    }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataSubtype {
  /// Data
  Data, // 0
  /// Data + CF-Ack
  DataCFAck, // 1
  /// Data + CF-Poll
  DataCFPoll, // 2
  /// Data + CF-Ack + CF-Poll
  DataCFAckCFPoll, // 3
  /// Null function (no data)
  Null, // 4
  /// CF-Ack (no data)
  CFAck, // 5
  /// CF-Poll (no data)
  CFPoll, // 6
  /// CF-Ack + CF-Poll (no data)
  CFAckCFPoll, // 7
  /// QoS Data
  QoSData, // 8
  /// Qos Data + CF-Ack
  QoSDataCFAck, // 9
  /// QoS Data + CF-Poll
  QoSDataCFPoll, // 10
  /// QoS Data + CF-Ack + CF-Poll
  QoSDataCFAckCFPoll, // 11
  /// QoS Null (no data)
  QoSNull, // 12
  /// 13 Reserved
  Reserved(u8), // 13
  /// QoS CF-Poll (no data)
  QoSCFPoll, // 14
  /// QoS CF-Ack + CF-Poll (no data)
  QoSCFAck, // 15
}

impl DataSubtype {
  pub fn from_u8(n: u8) -> DataSubtype {
    match n {
      0 => DataSubtype::Data,
      1 => DataSubtype::DataCFAck,
      2 => DataSubtype::DataCFPoll,
      3 => DataSubtype::DataCFAckCFPoll,
      4 => DataSubtype::Null,
      5 => DataSubtype::CFAck,
      6 => DataSubtype::CFPoll,
      7 => DataSubtype::CFAckCFPoll,
      8 => DataSubtype::QoSData,
      9 => DataSubtype::QoSDataCFAck,
      10 => DataSubtype::QoSDataCFPoll,
      11 => DataSubtype::QoSDataCFAckCFPoll,
      12 => DataSubtype::QoSNull,
      // 13 Reserved
      14 => DataSubtype::QoSCFPoll,
      15 => DataSubtype::QoSCFAck,
      other => DataSubtype::Reserved(other),
    }
  }

  pub fn to_u8(self) -> u8 {
    match self {
      DataSubtype::Data => 0,
      DataSubtype::DataCFAck => 1,
      DataSubtype::DataCFPoll => 2,
      DataSubtype::DataCFAckCFPoll => 3,
      DataSubtype::Null => 4,
      DataSubtype::CFAck => 5,
      DataSubtype::CFPoll => 6,
      DataSubtype::CFAckCFPoll => 7,
      DataSubtype::QoSData => 8,
      DataSubtype::QoSDataCFAck => 9,
      DataSubtype::QoSDataCFPoll => 10,
      DataSubtype::QoSDataCFAckCFPoll => 11,
      DataSubtype::QoSNull => 12,
      // 13 Reserved
      DataSubtype::QoSCFPoll => 14,
      DataSubtype::QoSCFAck => 15,
      DataSubtype::Reserved(other) => other,
    }
  }
}
