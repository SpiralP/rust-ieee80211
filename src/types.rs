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
  /// 0
  Management,
  /// 1
  Control,
  /// 2
  Data,
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

#[derive(Debug, PartialEq)]
pub enum StatusCode {
  ///  0 Successful
  Successful,
  ///  1 Unspecified failure
  UnspecifiedFailure,
  ///  2 TDLS wakeup schedule rejected but alternative schedule provided
  TDLSWakeupScheduleRejectedButAlternativeScheduleProvided,
  ///  3 TDLS wakeup schedule rejected
  TDLSWakeupScheduleRejected,
  ///  4 Reserved
  ///  5 Security disabled
  SecurityDisabled,
  ///  6 Unacceptable lifetime
  UnacceptableLifetime,
  ///  7 Not in same BSS
  NotInSameBSS,
  ///  8 Reserved
  ///  9 Reserved
  /// 10 Cannot support all requested capabilities in the Capability Information field
  CannotSupportAllRequestedCapabilities,
  /// 11 Reassociation denied due to inability to confirm that association exists
  ReassociationDenied,
  /// 12 Association denied due to reason outside the scope of this standard
  AssociationDenied,
  /// 13 Responding STA does not support the specified authentication algorithm
  STADoesNotSupportTheSpecifiedAuthenticationAlgorithm,
  /// 14 Received an Authentication frame with authentication transaction sequence number out of expected sequence
  ReceivedBadAuthenticationFrame,
  /// 15 Authentication rejected because of challenge failure
  AuthenticationRejectedBecauseOfChallengeFailure,
  /// 16 Authentication rejected due to timeout waiting for next frame in sequence
  AuthenticationRejectedDueToTimeoutWaitingForNextFrameInSequence,
  /// 17 Association denied because AP is unable to handle additional associated STAs
  AssociationDeniedBecauseAPIsUnableToHandleAdditionalAssociatedSTAs,
  /// 18 Association denied due to requesting STA not supporting all of the data rates in the BSSBasicRateSet parameter
  AssociationDeniedDataRates,
  /// 19 Association denied due to requesting STA not supporting the short preamble option
  AssociationDeniedShortPreamble,
  /// 20 Association denied due to requesting STA not supporting the PBCC modulation option
  AssociationDeniedPBCCModulation,
  /// 21 Association denied due to requesting STA not supporting the Channel Agility option
  AssociationDeniedChannelAgility,
  /// 22 Association request rejected because Spectrum Management capability is required
  AssociationRequestRejectedSpectrumManagement,
  /// 23 Association request rejected because the information in the Power Capability element is unacceptable
  AssociationRequestRejectedPowerCapability,
  /// 24 Association request rejected because the information in the Supported Channels element is unacceptable
  AssociationRequestRejectedSupportedChannels,
  /// 25 Association denied due to requesting STA not supporting the Short Slot Time option
  AssociationDeniedDueToRequestingSTANotSupportingTheShortSlotTimeOption,
  /// 26 Association denied due to requesting STA not supporting the DSSS-OFDM option
  AssociationDeniedDueToRequestingSTANotSupportingTheDSSSOFDMOption,
  /// 27 Reserved Association denied because the requesting STA does not support HT features
  ReservedAssociationDeniedBecauseTheRequestingSTADoesNotSupportHTFeatures,
  /// 28 R0KH unreachable
  R0KHUnreachable,
  /// 29 Association denied because the requesting STA does not support the phased coexistence operation (PCO) transition time required by the AP
  AssociationDeniedPhasedCoexistence,
  /// 30 Association request rejected temporarily; try again later
  AssociationRequestRejectedTemporarily,
  /// 31 Robust Management frame policy violation
  RobustManagementFramePolicyViolation,
  /// 32 Unspecified, QoS-related failure
  UnspecifiedQoSRelatedFailure,
  /// 33 Association denied because QoS AP or PCP has insufficient bandwidth to handle another QoS STA
  AssociationDeniedBecauseInsufficientBandwidth,
  /// 34 Association denied due to excessive frame loss rates and/or poor conditions on current operating channel
  AssociationDeniedExcessiveFrameLoss,
  /// 35 Association (with QoS BSS) denied because the requesting STA does not support the QoS facility
  AssociationWithQoSBSSDeniedBecauseTheRequestingSTADoesNotSupportTheQoSFacility,
  /// 36 Reserved
  /// 37 The request has been declined
  TheRequestHasBeenDeclined,
  /// 38 The request has not been successful as one or more parameters have invalid values
  TheRequestHasNotBeenSuccessfulAsOneOrMoreParametersHaveInvalidValues,
  /// 39 The allocation or TS has not been created because the request cannot be honored; however, a suggested TSPEC/DMG TSPEC is provided so that the initiating STA may attempt to set another allocation or TS with the suggested changes to the TSPEC/DMG TSPEC
  TheAllocationOrTSHasNotBeenCreated,
  /// 40 Invalid information element, i.e., an information element defined in this standard for which the content does not meet the specifications in Clause 7
  InvalidInformationElement,
  /// 41 Invalid group cipher
  InvalidGroupCipher,
  /// 42 Invalid pairwise cipher
  InvalidPairwiseCipher,
  /// 43 Invalid AKMP
  InvalidAKMP,
  /// 44 Unsupported RSN information element version
  UnsupportedRSNInformationElementVersion,
  /// 45 Invalid RSN information element capabilities
  InvalidRSNInformationElementCapabilities,
  /// 46 Cipher suite rejected because of security policy
  CipherSuiteRejectedBecauseOfSecurityPolicy,
  /// 47 The TS per allocation has not been created; however, the PCP/HC may be capable of creating a TS or allocation, in response to a request, after the time indicated in the TS Delay element
  TheTSPerAllocationHasNotBeenCreated,
  /// 48 Direct link is not allowed in the BSS by policy
  DirectLinkIsNotAllowedInTheBSSByPolicy,
  /// 49 The Destination STA is not present within this BSS
  TheDestinationSTAIsNotPresentWithinThisBSS,
  /// 50 The Destination STA is not a QoS STA
  TheDestinationSTAIsNotAQoSSTA,
  /// 51 Association denied because the ListenInterval is too large
  AssociationDeniedBecauseTheListenIntervalIsTooLarge,
  /// 52 Invalid FT Action frame count
  InvalidFTActionFrameCount,
  /// 53 Invalid pairwise master key identifier (PMKID)
  InvalidPairwiseMasterKeyIdentifier,
  /// 54 Invalid MDIE
  InvalidMDIE,
  /// 55 Invalid FTIE
  InvalidFTIE,
  /// 56 Requested TCLAS processing is not supported by the PCP/AP
  RequestedTCLASProcessingIsNotSupportedByThePCPOrAP,
  /// 57 The PCP/AP has insufficient TCLAS processing resources to satisfy the request
  ThePCPOrAPHasInsufficientTCLASProcessingResourcesToSatisfyTheRequest,
  /// 58 The TS has not been created because the request cannot be honored; however, the PCP/HC suggests the STA to transition to other BSSs to setup the TS
  TheTSHasNotBeenCreatedBecauseTheRequestCannotBeHonored,
  /// 59 GAS Advertisement Protocol not supported
  GASAdvertisementProtocolNotSupported,
  /// 60 No outstanding GAS request
  NoOutstandingGASRequest,
  /// 61 GAS Response not received from the Advertisement Server
  GASResponseNotReceivedFromTheAdvertisementServer,
  /// 62 STA timed out waiting for GAS Query Response
  STATimedOutWaitingForGASQueryResponse,
  /// 63 GAS Response is larger than query response length limit
  GASResponseIsLargerThanQueryResponseLengthLimit,
  /// 64 Request refused because home network does not support request
  RequestRefusedBecauseHomeNetworkDoesNotSupportRequest,
  /// 65 Advertisement Server in the network is not currently reachable
  AdvertisementServerInTheNetworkIsNotCurrentlyReachable,
  /// 66 Reserved
  /// 67 Request refused due to permissions received via SSPN interface
  RequestRefusedDueToPermissionsReceivedViaSSPNInterface,
  /// 68 Request refused because PCP/AP does not support unauthenticated access
  RequestRefusedBecausePCPOrAPDoesNotSupportUnauthenticatedAccess,
  /// 69 Reserved
  /// 70 Reserved
  /// 71 Reserved
  /// 72 Invalid contents of RSNIE
  InvalidContentsOfRSNIE,
  /// 73 U-APSD Coexistence is not supported
  UAPSDCoexistenceIsNotSupported,
  /// 74 Requested U-APSD Coexistence mode is not supported
  RequestedUAPSDCoexistenceModeIsNotSupported,
  /// 75 Requested Interval/Duration value cannot be supported with U-APSD Coexistence
  RequestedIntervalDurationValueCannotBeSupportedWithUAPSDCoexistence,
  /// 76 Authentication is rejected because an Anti-Clogging Token is required
  AuthenticationIsRejectedBecauseAnAntiCloggingTokenIsRequired,
  /// 77 Authentication is rejected because the offered finite cyclic group is not supported
  AuthenticationIsRejectedBecauseTheOfferedFiniteCyclicGroupIsNotSupported,
  /// 78 The TBTT adjustment request has not been successful because the STA could not find an alternative TBTT
  TheTBTTAdjustmentRequestHasNotBeenSuccessful,
  /// 79 Transmission failure
  TransmissionFailure,
  /// 80 Requested TCLAS Not Supported
  RequestedTCLASNotSupported,
  /// 81 TCLAS Resources Exhausted
  TCLASResourcesExhausted,
  /// 82 Rejected with Suggested BSS Transition
  RejectedWithSuggestedBSSTransition,
  /// 83 Reject with recommended schedule
  RejectWithRecommendedSchedule,
  /// 84 Reject because no wakeup schedule specified
  RejectBecauseNoWakeupScheduleSpecified,
  /// 85 Success, the destination STA is in power save mode
  SuccessTheDestinationSTAIsInPowerSaveMode,
  /// 86 FST pending, in process of admitting FST session
  FSTPendingInProcessOfAdmittingFSTSession,
  /// 87 performing FST now
  PerformingFSTNow,
  /// 88 FST pending, gap(s) in Block Ack window
  FSTPendingGapInBlockAckWindow,
  /// 89 Reject because of U-PID setting
  RejectBecauseOfUPIDSetting,
  /// 92 (Re)association refused for some external reason
  AssociationRefusedForSomeExternalReason,
  /// 93 (Re)association refused because of memory limits at the AP
  AssociationRefusedBecauseOfMemoryLimitsAtTheAP,
  /// 94 (Re)association refused because emergency services are not supported at the AP
  AssociationRefusedBecauseEmergencyServicesAreNotSupportedAtTheAP,
  /// 95 GAS query response not yet received
  GASQueryResponseNotYetReceived,
  /// 96 Reject since the request is for transition to a frequency band subject to DSE procedures and FST initiator is a dependent STA
  RejectDSEProcedures,
  /// 97 Reserved
  /// 98 Reserved
  /// 99 The association has been denied; however, one or more Multi-band elements are included that can be used by the receiving STA to join the BSS
  TheAssociationHasBeenDenied,
  /// 100 The request failed due to a reservation conflict
  TheRequestFailedDueToAReservationConflict,
  /// 101 The request failed due to exceeded MAF limit
  TheRequestFailedDueToExceededMAFLimit,
  /// 102 The request failed due to exceeded MCCA track limit
  TheRequestFailedDueToExceededMCCATrackLimit,
  /// 103 Association denied because the information in the Spectrum Management field is unacceptable
  AssociationDeniedBecauseSpectrumManagement,
  /// 104 Association denied because the requesting STA does not support VHT features
  AssociationDeniedBecauseTheRequestingSTADoesNotSupportVHTFeatures,
  /// 105 Enablement denied
  EnablementDenied,
  /// 106 Enablement denied due to restriction from an authorized GDB
  EnablementDeniedDueToRestrictionFromAnAuthorizedGDB,
  /// 107 Authorization deenabled
  AuthorizationDeenabled,
  /// 112 Authentication rejected due to FILS authentication failure
  AuthenticationRejectedDueToFILSAuthenticationFailure,
  /// 113 Authentication rejected due to unknown Authentication Server
  AuthenticationRejectedDueToUnknownAuthenticationServer,

  Reserved(u16),
}
impl StatusCode {
  pub fn from_u16(n: u16) -> Self {
    match n {
      0 => StatusCode::Successful,
      1 => StatusCode::UnspecifiedFailure,
      2 => StatusCode::TDLSWakeupScheduleRejectedButAlternativeScheduleProvided,
      3 => StatusCode::TDLSWakeupScheduleRejected,
      // 4 Reserved
      5 => StatusCode::SecurityDisabled,
      6 => StatusCode::UnacceptableLifetime,
      7 => StatusCode::NotInSameBSS,
      // 8 Reserved
      // 9 Reserved
      10 => StatusCode::CannotSupportAllRequestedCapabilities,
      11 => StatusCode::ReassociationDenied,
      12 => StatusCode::AssociationDenied,
      13 => StatusCode::STADoesNotSupportTheSpecifiedAuthenticationAlgorithm,
      14 => StatusCode::ReceivedBadAuthenticationFrame,
      15 => StatusCode::AuthenticationRejectedBecauseOfChallengeFailure,
      16 => StatusCode::AuthenticationRejectedDueToTimeoutWaitingForNextFrameInSequence,
      17 => StatusCode::AssociationDeniedBecauseAPIsUnableToHandleAdditionalAssociatedSTAs,
      18 => StatusCode::AssociationDeniedDataRates,
      19 => StatusCode::AssociationDeniedShortPreamble,
      20 => StatusCode::AssociationDeniedPBCCModulation,
      21 => StatusCode::AssociationDeniedChannelAgility,
      22 => StatusCode::AssociationRequestRejectedSpectrumManagement,
      23 => StatusCode::AssociationRequestRejectedPowerCapability,
      24 => StatusCode::AssociationRequestRejectedSupportedChannels,
      25 => StatusCode::AssociationDeniedDueToRequestingSTANotSupportingTheShortSlotTimeOption,
      26 => StatusCode::AssociationDeniedDueToRequestingSTANotSupportingTheDSSSOFDMOption,
      27 => StatusCode::ReservedAssociationDeniedBecauseTheRequestingSTADoesNotSupportHTFeatures,
      28 => StatusCode::R0KHUnreachable,
      29 => StatusCode::AssociationDeniedPhasedCoexistence,
      30 => StatusCode::AssociationRequestRejectedTemporarily,
      31 => StatusCode::RobustManagementFramePolicyViolation,
      32 => StatusCode::UnspecifiedQoSRelatedFailure,
      33 => StatusCode::AssociationDeniedBecauseInsufficientBandwidth,
      34 => StatusCode::AssociationDeniedExcessiveFrameLoss,
      35 => {
        StatusCode::AssociationWithQoSBSSDeniedBecauseTheRequestingSTADoesNotSupportTheQoSFacility
      }
      37 => StatusCode::TheRequestHasBeenDeclined,
      38 => StatusCode::TheRequestHasNotBeenSuccessfulAsOneOrMoreParametersHaveInvalidValues,
      39 => StatusCode::TheAllocationOrTSHasNotBeenCreated,
      40 => StatusCode::InvalidInformationElement,
      41 => StatusCode::InvalidGroupCipher,
      42 => StatusCode::InvalidPairwiseCipher,
      43 => StatusCode::InvalidAKMP,
      44 => StatusCode::UnsupportedRSNInformationElementVersion,
      45 => StatusCode::InvalidRSNInformationElementCapabilities,
      46 => StatusCode::CipherSuiteRejectedBecauseOfSecurityPolicy,
      47 => StatusCode::TheTSPerAllocationHasNotBeenCreated,
      48 => StatusCode::DirectLinkIsNotAllowedInTheBSSByPolicy,
      49 => StatusCode::TheDestinationSTAIsNotPresentWithinThisBSS,
      50 => StatusCode::TheDestinationSTAIsNotAQoSSTA,
      51 => StatusCode::AssociationDeniedBecauseTheListenIntervalIsTooLarge,
      52 => StatusCode::InvalidFTActionFrameCount,
      53 => StatusCode::InvalidPairwiseMasterKeyIdentifier,
      54 => StatusCode::InvalidMDIE,
      55 => StatusCode::InvalidFTIE,
      56 => StatusCode::RequestedTCLASProcessingIsNotSupportedByThePCPOrAP,
      57 => StatusCode::ThePCPOrAPHasInsufficientTCLASProcessingResourcesToSatisfyTheRequest,
      58 => StatusCode::TheTSHasNotBeenCreatedBecauseTheRequestCannotBeHonored,
      59 => StatusCode::GASAdvertisementProtocolNotSupported,
      60 => StatusCode::NoOutstandingGASRequest,
      61 => StatusCode::GASResponseNotReceivedFromTheAdvertisementServer,
      62 => StatusCode::STATimedOutWaitingForGASQueryResponse,
      63 => StatusCode::GASResponseIsLargerThanQueryResponseLengthLimit,
      64 => StatusCode::RequestRefusedBecauseHomeNetworkDoesNotSupportRequest,
      65 => StatusCode::AdvertisementServerInTheNetworkIsNotCurrentlyReachable,
      67 => StatusCode::RequestRefusedDueToPermissionsReceivedViaSSPNInterface,
      68 => StatusCode::RequestRefusedBecausePCPOrAPDoesNotSupportUnauthenticatedAccess,
      72 => StatusCode::InvalidContentsOfRSNIE,
      73 => StatusCode::UAPSDCoexistenceIsNotSupported,
      74 => StatusCode::RequestedUAPSDCoexistenceModeIsNotSupported,
      75 => StatusCode::RequestedIntervalDurationValueCannotBeSupportedWithUAPSDCoexistence,
      76 => StatusCode::AuthenticationIsRejectedBecauseAnAntiCloggingTokenIsRequired,
      77 => StatusCode::AuthenticationIsRejectedBecauseTheOfferedFiniteCyclicGroupIsNotSupported,
      78 => StatusCode::TheTBTTAdjustmentRequestHasNotBeenSuccessful,
      79 => StatusCode::TransmissionFailure,
      80 => StatusCode::RequestedTCLASNotSupported,
      81 => StatusCode::TCLASResourcesExhausted,
      82 => StatusCode::RejectedWithSuggestedBSSTransition,
      83 => StatusCode::RejectWithRecommendedSchedule,
      84 => StatusCode::RejectBecauseNoWakeupScheduleSpecified,
      85 => StatusCode::SuccessTheDestinationSTAIsInPowerSaveMode,
      86 => StatusCode::FSTPendingInProcessOfAdmittingFSTSession,
      87 => StatusCode::PerformingFSTNow,
      88 => StatusCode::FSTPendingGapInBlockAckWindow,
      89 => StatusCode::RejectBecauseOfUPIDSetting,
      92 => StatusCode::AssociationRefusedForSomeExternalReason,
      93 => StatusCode::AssociationRefusedBecauseOfMemoryLimitsAtTheAP,
      94 => StatusCode::AssociationRefusedBecauseEmergencyServicesAreNotSupportedAtTheAP,
      95 => StatusCode::GASQueryResponseNotYetReceived,
      96 => StatusCode::RejectDSEProcedures,
      99 => StatusCode::TheAssociationHasBeenDenied,
      100 => StatusCode::TheRequestFailedDueToAReservationConflict,
      101 => StatusCode::TheRequestFailedDueToExceededMAFLimit,
      102 => StatusCode::TheRequestFailedDueToExceededMCCATrackLimit,
      103 => StatusCode::AssociationDeniedBecauseSpectrumManagement,
      104 => StatusCode::AssociationDeniedBecauseTheRequestingSTADoesNotSupportVHTFeatures,
      105 => StatusCode::EnablementDenied,
      106 => StatusCode::EnablementDeniedDueToRestrictionFromAnAuthorizedGDB,
      107 => StatusCode::AuthorizationDeenabled,
      112 => StatusCode::AuthenticationRejectedDueToFILSAuthenticationFailure,
      113 => StatusCode::AuthenticationRejectedDueToUnknownAuthenticationServer,
      other => StatusCode::Reserved(other),
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum ReasonCode {
  Reserved(u16),

  ///  1 Unspecified reason
  UnspecifiedReason,
  ///  2 Previous authentication no longer valid
  PreviousAuthenticationNoLongerValid,
  ///  3 Deauthenticated because sending STA is leaving (or has left) IBSS or ESS
  STALeavingIBSSOrESS,
  ///  4 Disassociated due to inactivity
  Inactivity,
  ///  5 Disassociated because AP is unable to handle all currently associated STAs
  APIsUnableToHandleAllCurrentlyAssociatedSTAs,
  ///  6 Class 2 frame received from nonauthenticated STA
  Class2FrameReceivedFromNonauthenticatedSTA,
  ///  7 Class 3 frame received from nonassociated STA
  Class3FrameReceivedFromNonassociatedSTA,
  ///  8 Disassociated because sending STA is leaving (or has left) BSS
  STALeavingBSS,
  ///  9 STA requesting (re)association is not authenticated with responding STA
  NotAuthenticated,
  /// 10 Disassociated because the information in the Power Capability element is unacceptable
  PowerCapabilityElementIsUnacceptable,
  /// 11 Disassociated because the information in the Supported Channels element is unacceptable
  SupportedChannelsElementIsUnacceptable,
  /// 12 Disassociated due to BSS transition management
  DisassociatedDueToBSSTransitionManagement,
  /// 13 Invalid information element, i.e., an information element defined in this standard for which the content does not meet the specifications in Clause 7
  InvalidInformationElement,
  /// 14 Message integrity code (MIC) failure
  MessageIntegrityCodeFailure,
  /// 15 4-Way Handshake timeout
  FourWayHandshakeTimeout,
  /// 16 Group Key Handshake timeout
  GroupKeyHandshakeTimeout,
  /// 17 Information element in 4-Way Handshake different from (Re)Association Request/Probe Response/Beacon frame
  InformationDifferent,
  /// 18 Invalid group cipher
  InvalidGroupCipher,
  /// 19 Invalid pairwise cipher
  InvalidPairwiseCipher,
  /// 20 Invalid AKMP
  InvalidAKMP,
  /// 21 Unsupported RSN information element version
  UnsupportedRSNInformationElementVersion,
  /// 22 Invalid RSN information element capabilities
  InvalidRSNInformationElementCapabilities,
  /// 23 IEEE 802.1X authentication failed
  IEEE8021XAuthenticationFailed,
  /// 24 Cipher suite rejected because of the security policy
  CipherSuiteRejectedBecauseOfTheSecurityPolicy,
  /// 25 TDLS direct-link teardown due to TDLS peer STA unreachable via the TDLS direct link
  TDLSDirectLinkTeardownUnreachable,
  /// 26 TDLS direct-link teardown for unspecified reason
  TDLSDirectLinkTeardownUnspecifiedReason,
  /// 27 Disassociated because session terminated by SSP request
  DisassociatedBecauseSessionTerminatedBySSPRequest,
  /// 28 Disassociated because of lack of SSP roaming agreement
  DisassociatedBecauseOfLackOfSSPRoamingAgreement,
  /// 29 Requested service rejected because of SSP cipher suite or AKM requirement
  RequestedServiceRejected,
  /// 30 Requested service not authorized in this location
  RequestedServiceNotAuthorizedInThisLocation,
  /// 31 TS deleted because QoS AP lacks sufficient bandwidth for this QoS STA due to a change in BSS service characteristics or operational mode
  TSDeleted,
  /// 32 Disassociated for unspecified, QoS-related reason
  DisassociatedForUnspecifiedQoSRelatedReason,
  /// 33 Disassociated because QoS AP lacks sufficient bandwidth for this QoS STA
  QoSAPLacksSufficientBandwidthForThisQoSSTA,
  /// 34 Disassociated because excessive number of frames need to be acknowledged, but are not acknowledged due to AP transmissions and/or poor channel conditions
  ExcessiveNumberOfFramesNotAcknowledged,
  /// 35 Disassociated because STA is transmitting outside the limits of its TXOPs
  STAIsTransmittingOutsideTheLimitsOfItsTXOPs,
  /// 36 Requested from peer STA as the STA is leaving the BSS (or resetting)
  STALeavingTheBSS,
  /// 37 Requested from peer STA as it does not want to use the mechanism
  STADoesNotWantToUseTheMechanism,
  /// 38 Requested from peer STA as the STA received frames using the mechanism for which a setup is required
  STAReceivedFramesUsingTheMechanismForWhichASetupIsRequired,
  /// 39 Requested from peer STA due to timeout
  RequestedFromPeerSTADueToTimeout,
  /// 45 Peer STA does not support the requested cipher suite
  PeerSTADoesNotSupportTheRequestedCipherSuite,
  /// 46 Disassociated because authorized access limit reached
  AuthorizedAccessLimitReached,
  /// 47 Disassociated due to external service requirements
  DisassociatedDueToExternalServiceRequirements,
  /// 48 Invalid FT Action frame count
  InvalidFTActionFrameCount,
  /// 49 Invalid pairwise master key identifier (PMKI)
  InvalidPairwiseMasterKeyIdentifier,
  /// 50 Invalid MDE
  InvalidMDE,
  /// 51 Invalid FTE
  InvalidFTE,
  /// 52 SME cancels the mesh peering instance with the reason other than reaching the maximum number of peer mesh STAs
  SMECancel,
  /// 53 The mesh STA has reached the supported maximum number of peer mesh STAs
  MeshMaxSupportedMaximumNumberOfPeerMeshSTAs,
  /// 54 The received information violates the Mesh Configuration policy configured in the mesh STA profile
  InformationViolatesMeshPolicy,
  /// 55 The mesh STA has received a Mesh Peering Close message requesting to close the mesh peering
  MeshSTAReceivedAMeshPeeringCloseMessage,
  /// 56 The mesh STA has re-sent dot11MeshMaxRetries Mesh Peering Open messages, without receiving a Mesh Peering Confirm message
  MeshSTAHasResentMaxRetriesWithoutConfirm,
  /// 57 The confirmTimer for the mesh peering instance times out
  TheConfirmTimerForTheMeshPeeringInstanceTimesOut,
  /// 58 The mesh STA fails to unwrap the GTK or the values in the wrapped contents do not match
  MeshSTAFailsToUnwrapGTK,
  /// 59 The mesh STA receives inconsistent information about the mesh parameters between Mesh Peering Management frames
  MeshSTAReceivesInconsistentInformation,
  /// 60 The mesh STA fails the authenticated mesh peering exchange because due to failure in selecting either the pairwise ciphersuite or group ciphersuite
  MeshSTAFailsAuthenticatedMeshPeeringExchange,
  /// 61 The mesh STA does not have proxy information for this external destination
  MeshSTADoesNotHaveProxyInformation,
  /// 62 The mesh STA does not have forwarding information for this destination
  MeshSTADoesNotHaveForwardingInformation,
  /// 63 The mesh STA determines that the link to the next hop of an active path in its forwarding information is no longer usable
  MeshSTALinkNoLongerUsable,
  /// 64 The Deauthentication frame was sent because the MAC address of the STA already exists in the mesh BSS. See 11.3.3 (Additional mechanisms for an AP collocated with a mesh STA)
  MacAddressAlreadyExists,
  /// 65 The mesh STA performs channel switch to meet regulatory requirements
  MeshSTAPerformsChannelSwitchToMeetRegulatoryRequirements,
  /// 66 The mesh STA performs channel switch with unspecified reason
  MeshSTAPerformsChannelSwitchWithUnspecifiedReason,
}
impl ReasonCode {
  pub fn from_u16(n: u16) -> Self {
    match n {
      1 => ReasonCode::UnspecifiedReason,
      2 => ReasonCode::PreviousAuthenticationNoLongerValid,
      3 => ReasonCode::STALeavingIBSSOrESS,
      4 => ReasonCode::Inactivity,
      5 => ReasonCode::APIsUnableToHandleAllCurrentlyAssociatedSTAs,
      6 => ReasonCode::Class2FrameReceivedFromNonauthenticatedSTA,
      7 => ReasonCode::Class3FrameReceivedFromNonassociatedSTA,
      8 => ReasonCode::STALeavingBSS,
      9 => ReasonCode::NotAuthenticated,
      10 => ReasonCode::PowerCapabilityElementIsUnacceptable,
      11 => ReasonCode::SupportedChannelsElementIsUnacceptable,
      12 => ReasonCode::DisassociatedDueToBSSTransitionManagement,
      13 => ReasonCode::InvalidInformationElement,
      14 => ReasonCode::MessageIntegrityCodeFailure,
      15 => ReasonCode::FourWayHandshakeTimeout,
      16 => ReasonCode::GroupKeyHandshakeTimeout,
      17 => ReasonCode::InformationDifferent,
      18 => ReasonCode::InvalidGroupCipher,
      19 => ReasonCode::InvalidPairwiseCipher,
      20 => ReasonCode::InvalidAKMP,
      21 => ReasonCode::UnsupportedRSNInformationElementVersion,
      22 => ReasonCode::InvalidRSNInformationElementCapabilities,
      23 => ReasonCode::IEEE8021XAuthenticationFailed,
      24 => ReasonCode::CipherSuiteRejectedBecauseOfTheSecurityPolicy,
      25 => ReasonCode::TDLSDirectLinkTeardownUnreachable,
      26 => ReasonCode::TDLSDirectLinkTeardownUnspecifiedReason,
      27 => ReasonCode::DisassociatedBecauseSessionTerminatedBySSPRequest,
      28 => ReasonCode::DisassociatedBecauseOfLackOfSSPRoamingAgreement,
      29 => ReasonCode::RequestedServiceRejected,
      30 => ReasonCode::RequestedServiceNotAuthorizedInThisLocation,
      31 => ReasonCode::TSDeleted,
      32 => ReasonCode::DisassociatedForUnspecifiedQoSRelatedReason,
      33 => ReasonCode::QoSAPLacksSufficientBandwidthForThisQoSSTA,
      34 => ReasonCode::ExcessiveNumberOfFramesNotAcknowledged,
      35 => ReasonCode::STAIsTransmittingOutsideTheLimitsOfItsTXOPs,
      36 => ReasonCode::STALeavingTheBSS,
      37 => ReasonCode::STADoesNotWantToUseTheMechanism,
      38 => ReasonCode::STAReceivedFramesUsingTheMechanismForWhichASetupIsRequired,
      39 => ReasonCode::RequestedFromPeerSTADueToTimeout,
      45 => ReasonCode::PeerSTADoesNotSupportTheRequestedCipherSuite,
      46 => ReasonCode::AuthorizedAccessLimitReached,
      47 => ReasonCode::DisassociatedDueToExternalServiceRequirements,
      48 => ReasonCode::InvalidFTActionFrameCount,
      49 => ReasonCode::InvalidPairwiseMasterKeyIdentifier,
      50 => ReasonCode::InvalidMDE,
      51 => ReasonCode::InvalidFTE,
      52 => ReasonCode::SMECancel,
      53 => ReasonCode::MeshMaxSupportedMaximumNumberOfPeerMeshSTAs,
      54 => ReasonCode::InformationViolatesMeshPolicy,
      55 => ReasonCode::MeshSTAReceivedAMeshPeeringCloseMessage,
      56 => ReasonCode::MeshSTAHasResentMaxRetriesWithoutConfirm,
      57 => ReasonCode::TheConfirmTimerForTheMeshPeeringInstanceTimesOut,
      58 => ReasonCode::MeshSTAFailsToUnwrapGTK,
      59 => ReasonCode::MeshSTAReceivesInconsistentInformation,
      60 => ReasonCode::MeshSTAFailsAuthenticatedMeshPeeringExchange,
      61 => ReasonCode::MeshSTADoesNotHaveProxyInformation,
      62 => ReasonCode::MeshSTADoesNotHaveForwardingInformation,
      63 => ReasonCode::MeshSTALinkNoLongerUsable,
      64 => ReasonCode::MacAddressAlreadyExists,
      65 => ReasonCode::MeshSTAPerformsChannelSwitchToMeetRegulatoryRequirements,
      66 => ReasonCode::MeshSTAPerformsChannelSwitchWithUnspecifiedReason,

      other => ReasonCode::Reserved(other),
    }
  }
}
