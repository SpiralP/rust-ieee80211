#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    #[must_use]
    pub fn from_bools(from_ds: bool, to_ds: bool) -> Self {
        match (from_ds, to_ds) {
            // 00 Not leaving DS or network is operating in AD-HOC mode
            (false, false) => Self::NotLeavingDSOrADHOC,
            // 01 Frame from STA to DS via an AP
            (false, true) => Self::FromSTAToDS,
            // 10 Frame from DS to a STA via AP
            (true, false) => Self::FromDSToSTA,
            // 11 WDS (AP to AP) or Mesh (MP to MP) Frame
            (true, true) => Self::WDSOrMesh,
        }
    }

    /// returns (`from_ds`, `to_ds`)
    #[must_use]
    pub fn to_bools(self) -> (bool, bool) {
        match self {
            // 00 Not leaving DS or network is operating in AD-HOC mode
            Self::NotLeavingDSOrADHOC => (false, false),
            // 01 Frame from STA to DS via an AP
            Self::FromSTAToDS => (false, true),
            // 10 Frame from DS to a STA via AP
            Self::FromDSToSTA => (true, false),
            // 11 WDS (AP to AP) or Mesh (MP to MP) Frame
            Self::WDSOrMesh => (true, true),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DurationID {
    /// Microseconds
    Duration(u16),
    /// Association Identifier (AID)
    /// valid range 1-2007
    AssociationID(u16),

    Reserved(u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrameVersion {
    Standard, // 0
    Reserved(u8),
}

impl FrameVersion {
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Standard,
            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Standard => 0,
            Self::Reserved(other) => other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Management,
            1 => Self::Control,
            2 => Self::Data,
            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Management => 0,
            Self::Control => 1,
            Self::Data => 2,
            Self::Reserved(other) => other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FrameSubtype {
    Management(ManagementSubtype),
    Control(ControlSubtype),
    Data(DataSubtype),

    /// main type, subtype
    Reserved(u8, u8),
}

impl FrameSubtype {
    #[must_use]
    pub fn from_u8(type_: FrameType, subtype: u8) -> Self {
        match type_ {
            FrameType::Management => Self::Management(ManagementSubtype::from_u8(subtype)),
            FrameType::Control => Self::Control(ControlSubtype::from_u8(subtype)),
            FrameType::Data => Self::Data(DataSubtype::from_u8(subtype)),
            FrameType::Reserved(type_) => Self::Reserved(type_, subtype),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Management(subtype) => subtype.into_u8(),
            Self::Control(subtype) => subtype.into_u8(),
            Self::Data(subtype) => subtype.into_u8(),
            Self::Reserved(_type, subtype) => subtype,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::AssociationRequest,
            1 => Self::AssociationResponse,
            2 => Self::ReassociationRequest,
            3 => Self::ReassociationResponse,
            4 => Self::ProbeRequest,
            5 => Self::ProbeResponse,
            // 6-7 Reserved
            8 => Self::Beacon,
            9 => Self::ATIM,
            10 => Self::Disassociate,
            11 => Self::Authentication,
            12 => Self::Deauthentication,
            13 => Self::Action,
            14 => Self::ActionNoAck,
            // 15 Reserved
            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            Self::AssociationRequest => 0,
            Self::AssociationResponse => 1,
            Self::ReassociationRequest => 2,
            Self::ReassociationResponse => 3,
            Self::ProbeRequest => 4,
            Self::ProbeResponse => 5,
            // 6-7 Reserved
            Self::Beacon => 8,
            Self::ATIM => 9,
            Self::Disassociate => 10,
            Self::Authentication => 11,
            Self::Deauthentication => 12,
            Self::Action => 13,
            Self::ActionNoAck => 14,
            // 15 Reserved
            Self::Reserved(other) => other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        match n {
            // 0-6 Reserved
            7 => Self::ControlWrapper,
            8 => Self::BlockAckRequest,
            9 => Self::BlockAck,
            10 => Self::PSPoll,
            11 => Self::RTS,
            12 => Self::CTS,
            13 => Self::Ack,
            14 => Self::CFEnd,
            15 => Self::CFEndCFAck,
            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            // 0-6 Reserved
            Self::ControlWrapper => 7,
            Self::BlockAckRequest => 8,
            Self::BlockAck => 9,
            Self::PSPoll => 10,
            Self::RTS => 11,
            Self::CTS => 12,
            Self::Ack => 13,
            Self::CFEnd => 14,
            Self::CFEndCFAck => 15,
            Self::Reserved(other) => other,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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
    #[must_use]
    pub fn from_u8(n: u8) -> Self {
        match n {
            0 => Self::Data,
            1 => Self::DataCFAck,
            2 => Self::DataCFPoll,
            3 => Self::DataCFAckCFPoll,
            4 => Self::Null,
            5 => Self::CFAck,
            6 => Self::CFPoll,
            7 => Self::CFAckCFPoll,
            8 => Self::QoSData,
            9 => Self::QoSDataCFAck,
            10 => Self::QoSDataCFPoll,
            11 => Self::QoSDataCFAckCFPoll,
            12 => Self::QoSNull,
            // 13 Reserved
            14 => Self::QoSCFPoll,
            15 => Self::QoSCFAck,
            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Data => 0,
            Self::DataCFAck => 1,
            Self::DataCFPoll => 2,
            Self::DataCFAckCFPoll => 3,
            Self::Null => 4,
            Self::CFAck => 5,
            Self::CFPoll => 6,
            Self::CFAckCFPoll => 7,
            Self::QoSData => 8,
            Self::QoSDataCFAck => 9,
            Self::QoSDataCFPoll => 10,
            Self::QoSDataCFAckCFPoll => 11,
            Self::QoSNull => 12,
            // 13 Reserved
            Self::QoSCFPoll => 14,
            Self::QoSCFAck => 15,
            Self::Reserved(other) => other,
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
    AssociationWithQoSBSSDenied,
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
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn from_u16(n: u16) -> Self {
        match n {
            0 => Self::Successful,
            1 => Self::UnspecifiedFailure,
            2 => Self::TDLSWakeupScheduleRejectedButAlternativeScheduleProvided,
            3 => Self::TDLSWakeupScheduleRejected,
            // 4 Reserved
            5 => Self::SecurityDisabled,
            6 => Self::UnacceptableLifetime,
            7 => Self::NotInSameBSS,
            // 8 Reserved
            // 9 Reserved
            10 => Self::CannotSupportAllRequestedCapabilities,
            11 => Self::ReassociationDenied,
            12 => Self::AssociationDenied,
            13 => Self::STADoesNotSupportTheSpecifiedAuthenticationAlgorithm,
            14 => Self::ReceivedBadAuthenticationFrame,
            15 => Self::AuthenticationRejectedBecauseOfChallengeFailure,
            16 => Self::AuthenticationRejectedDueToTimeoutWaitingForNextFrameInSequence,
            17 => Self::AssociationDeniedBecauseAPIsUnableToHandleAdditionalAssociatedSTAs,
            18 => Self::AssociationDeniedDataRates,
            19 => Self::AssociationDeniedShortPreamble,
            20 => Self::AssociationDeniedPBCCModulation,
            21 => Self::AssociationDeniedChannelAgility,
            22 => Self::AssociationRequestRejectedSpectrumManagement,
            23 => Self::AssociationRequestRejectedPowerCapability,
            24 => Self::AssociationRequestRejectedSupportedChannels,
            25 => Self::AssociationDeniedDueToRequestingSTANotSupportingTheShortSlotTimeOption,
            26 => Self::AssociationDeniedDueToRequestingSTANotSupportingTheDSSSOFDMOption,
            27 => Self::ReservedAssociationDeniedBecauseTheRequestingSTADoesNotSupportHTFeatures,
            28 => Self::R0KHUnreachable,
            29 => Self::AssociationDeniedPhasedCoexistence,
            30 => Self::AssociationRequestRejectedTemporarily,
            31 => Self::RobustManagementFramePolicyViolation,
            32 => Self::UnspecifiedQoSRelatedFailure,
            33 => Self::AssociationDeniedBecauseInsufficientBandwidth,
            34 => Self::AssociationDeniedExcessiveFrameLoss,
            35 => Self::AssociationWithQoSBSSDenied,
            37 => Self::TheRequestHasBeenDeclined,
            38 => Self::TheRequestHasNotBeenSuccessfulAsOneOrMoreParametersHaveInvalidValues,
            39 => Self::TheAllocationOrTSHasNotBeenCreated,
            40 => Self::InvalidInformationElement,
            41 => Self::InvalidGroupCipher,
            42 => Self::InvalidPairwiseCipher,
            43 => Self::InvalidAKMP,
            44 => Self::UnsupportedRSNInformationElementVersion,
            45 => Self::InvalidRSNInformationElementCapabilities,
            46 => Self::CipherSuiteRejectedBecauseOfSecurityPolicy,
            47 => Self::TheTSPerAllocationHasNotBeenCreated,
            48 => Self::DirectLinkIsNotAllowedInTheBSSByPolicy,
            49 => Self::TheDestinationSTAIsNotPresentWithinThisBSS,
            50 => Self::TheDestinationSTAIsNotAQoSSTA,
            51 => Self::AssociationDeniedBecauseTheListenIntervalIsTooLarge,
            52 => Self::InvalidFTActionFrameCount,
            53 => Self::InvalidPairwiseMasterKeyIdentifier,
            54 => Self::InvalidMDIE,
            55 => Self::InvalidFTIE,
            56 => Self::RequestedTCLASProcessingIsNotSupportedByThePCPOrAP,
            57 => Self::ThePCPOrAPHasInsufficientTCLASProcessingResourcesToSatisfyTheRequest,
            58 => Self::TheTSHasNotBeenCreatedBecauseTheRequestCannotBeHonored,
            59 => Self::GASAdvertisementProtocolNotSupported,
            60 => Self::NoOutstandingGASRequest,
            61 => Self::GASResponseNotReceivedFromTheAdvertisementServer,
            62 => Self::STATimedOutWaitingForGASQueryResponse,
            63 => Self::GASResponseIsLargerThanQueryResponseLengthLimit,
            64 => Self::RequestRefusedBecauseHomeNetworkDoesNotSupportRequest,
            65 => Self::AdvertisementServerInTheNetworkIsNotCurrentlyReachable,
            67 => Self::RequestRefusedDueToPermissionsReceivedViaSSPNInterface,
            68 => Self::RequestRefusedBecausePCPOrAPDoesNotSupportUnauthenticatedAccess,
            72 => Self::InvalidContentsOfRSNIE,
            73 => Self::UAPSDCoexistenceIsNotSupported,
            74 => Self::RequestedUAPSDCoexistenceModeIsNotSupported,
            75 => Self::RequestedIntervalDurationValueCannotBeSupportedWithUAPSDCoexistence,
            76 => Self::AuthenticationIsRejectedBecauseAnAntiCloggingTokenIsRequired,
            77 => Self::AuthenticationIsRejectedBecauseTheOfferedFiniteCyclicGroupIsNotSupported,
            78 => Self::TheTBTTAdjustmentRequestHasNotBeenSuccessful,
            79 => Self::TransmissionFailure,
            80 => Self::RequestedTCLASNotSupported,
            81 => Self::TCLASResourcesExhausted,
            82 => Self::RejectedWithSuggestedBSSTransition,
            83 => Self::RejectWithRecommendedSchedule,
            84 => Self::RejectBecauseNoWakeupScheduleSpecified,
            85 => Self::SuccessTheDestinationSTAIsInPowerSaveMode,
            86 => Self::FSTPendingInProcessOfAdmittingFSTSession,
            87 => Self::PerformingFSTNow,
            88 => Self::FSTPendingGapInBlockAckWindow,
            89 => Self::RejectBecauseOfUPIDSetting,
            92 => Self::AssociationRefusedForSomeExternalReason,
            93 => Self::AssociationRefusedBecauseOfMemoryLimitsAtTheAP,
            94 => Self::AssociationRefusedBecauseEmergencyServicesAreNotSupportedAtTheAP,
            95 => Self::GASQueryResponseNotYetReceived,
            96 => Self::RejectDSEProcedures,
            99 => Self::TheAssociationHasBeenDenied,
            100 => Self::TheRequestFailedDueToAReservationConflict,
            101 => Self::TheRequestFailedDueToExceededMAFLimit,
            102 => Self::TheRequestFailedDueToExceededMCCATrackLimit,
            103 => Self::AssociationDeniedBecauseSpectrumManagement,
            104 => Self::AssociationDeniedBecauseTheRequestingSTADoesNotSupportVHTFeatures,
            105 => Self::EnablementDenied,
            106 => Self::EnablementDeniedDueToRestrictionFromAnAuthorizedGDB,
            107 => Self::AuthorizationDeenabled,
            112 => Self::AuthenticationRejectedDueToFILSAuthenticationFailure,
            113 => Self::AuthenticationRejectedDueToUnknownAuthenticationServer,

            other => Self::Reserved(other),
        }
    }

    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn into_u16(self) -> u16 {
        match self {
            Self::Successful => 0,
            Self::UnspecifiedFailure => 1,
            Self::TDLSWakeupScheduleRejectedButAlternativeScheduleProvided => 2,
            Self::TDLSWakeupScheduleRejected => 3,
            // 4 Reserved
            Self::SecurityDisabled => 5,
            Self::UnacceptableLifetime => 6,
            Self::NotInSameBSS => 7,
            // 8 Reserved
            // 9 Reserved
            Self::CannotSupportAllRequestedCapabilities => 10,
            Self::ReassociationDenied => 11,
            Self::AssociationDenied => 12,
            Self::STADoesNotSupportTheSpecifiedAuthenticationAlgorithm => 13,
            Self::ReceivedBadAuthenticationFrame => 14,
            Self::AuthenticationRejectedBecauseOfChallengeFailure => 15,
            Self::AuthenticationRejectedDueToTimeoutWaitingForNextFrameInSequence => 16,
            Self::AssociationDeniedBecauseAPIsUnableToHandleAdditionalAssociatedSTAs => 17,
            Self::AssociationDeniedDataRates => 18,
            Self::AssociationDeniedShortPreamble => 19,
            Self::AssociationDeniedPBCCModulation => 20,
            Self::AssociationDeniedChannelAgility => 21,
            Self::AssociationRequestRejectedSpectrumManagement => 22,
            Self::AssociationRequestRejectedPowerCapability => 23,
            Self::AssociationRequestRejectedSupportedChannels => 24,
            Self::AssociationDeniedDueToRequestingSTANotSupportingTheShortSlotTimeOption => 25,
            Self::AssociationDeniedDueToRequestingSTANotSupportingTheDSSSOFDMOption => 26,
            Self::ReservedAssociationDeniedBecauseTheRequestingSTADoesNotSupportHTFeatures => 27,
            Self::R0KHUnreachable => 28,
            Self::AssociationDeniedPhasedCoexistence => 29,
            Self::AssociationRequestRejectedTemporarily => 30,
            Self::RobustManagementFramePolicyViolation => 31,
            Self::UnspecifiedQoSRelatedFailure => 32,
            Self::AssociationDeniedBecauseInsufficientBandwidth => 33,
            Self::AssociationDeniedExcessiveFrameLoss => 34,
            Self::AssociationWithQoSBSSDenied => 35,
            Self::TheRequestHasBeenDeclined => 37,
            Self::TheRequestHasNotBeenSuccessfulAsOneOrMoreParametersHaveInvalidValues => 38,
            Self::TheAllocationOrTSHasNotBeenCreated => 39,
            Self::InvalidInformationElement => 40,
            Self::InvalidGroupCipher => 41,
            Self::InvalidPairwiseCipher => 42,
            Self::InvalidAKMP => 43,
            Self::UnsupportedRSNInformationElementVersion => 44,
            Self::InvalidRSNInformationElementCapabilities => 45,
            Self::CipherSuiteRejectedBecauseOfSecurityPolicy => 46,
            Self::TheTSPerAllocationHasNotBeenCreated => 47,
            Self::DirectLinkIsNotAllowedInTheBSSByPolicy => 48,
            Self::TheDestinationSTAIsNotPresentWithinThisBSS => 49,
            Self::TheDestinationSTAIsNotAQoSSTA => 50,
            Self::AssociationDeniedBecauseTheListenIntervalIsTooLarge => 51,
            Self::InvalidFTActionFrameCount => 52,
            Self::InvalidPairwiseMasterKeyIdentifier => 53,
            Self::InvalidMDIE => 54,
            Self::InvalidFTIE => 55,
            Self::RequestedTCLASProcessingIsNotSupportedByThePCPOrAP => 56,
            Self::ThePCPOrAPHasInsufficientTCLASProcessingResourcesToSatisfyTheRequest => 57,
            Self::TheTSHasNotBeenCreatedBecauseTheRequestCannotBeHonored => 58,
            Self::GASAdvertisementProtocolNotSupported => 59,
            Self::NoOutstandingGASRequest => 60,
            Self::GASResponseNotReceivedFromTheAdvertisementServer => 61,
            Self::STATimedOutWaitingForGASQueryResponse => 62,
            Self::GASResponseIsLargerThanQueryResponseLengthLimit => 63,
            Self::RequestRefusedBecauseHomeNetworkDoesNotSupportRequest => 64,
            Self::AdvertisementServerInTheNetworkIsNotCurrentlyReachable => 65,
            Self::RequestRefusedDueToPermissionsReceivedViaSSPNInterface => 67,
            Self::RequestRefusedBecausePCPOrAPDoesNotSupportUnauthenticatedAccess => 68,
            Self::InvalidContentsOfRSNIE => 72,
            Self::UAPSDCoexistenceIsNotSupported => 73,
            Self::RequestedUAPSDCoexistenceModeIsNotSupported => 74,
            Self::RequestedIntervalDurationValueCannotBeSupportedWithUAPSDCoexistence => 75,
            Self::AuthenticationIsRejectedBecauseAnAntiCloggingTokenIsRequired => 76,
            Self::AuthenticationIsRejectedBecauseTheOfferedFiniteCyclicGroupIsNotSupported => 77,
            Self::TheTBTTAdjustmentRequestHasNotBeenSuccessful => 78,
            Self::TransmissionFailure => 79,
            Self::RequestedTCLASNotSupported => 80,
            Self::TCLASResourcesExhausted => 81,
            Self::RejectedWithSuggestedBSSTransition => 82,
            Self::RejectWithRecommendedSchedule => 83,
            Self::RejectBecauseNoWakeupScheduleSpecified => 84,
            Self::SuccessTheDestinationSTAIsInPowerSaveMode => 85,
            Self::FSTPendingInProcessOfAdmittingFSTSession => 86,
            Self::PerformingFSTNow => 87,
            Self::FSTPendingGapInBlockAckWindow => 88,
            Self::RejectBecauseOfUPIDSetting => 89,
            Self::AssociationRefusedForSomeExternalReason => 92,
            Self::AssociationRefusedBecauseOfMemoryLimitsAtTheAP => 93,
            Self::AssociationRefusedBecauseEmergencyServicesAreNotSupportedAtTheAP => 94,
            Self::GASQueryResponseNotYetReceived => 95,
            Self::RejectDSEProcedures => 96,
            Self::TheAssociationHasBeenDenied => 99,
            Self::TheRequestFailedDueToAReservationConflict => 100,
            Self::TheRequestFailedDueToExceededMAFLimit => 101,
            Self::TheRequestFailedDueToExceededMCCATrackLimit => 102,
            Self::AssociationDeniedBecauseSpectrumManagement => 103,
            Self::AssociationDeniedBecauseTheRequestingSTADoesNotSupportVHTFeatures => 104,
            Self::EnablementDenied => 105,
            Self::EnablementDeniedDueToRestrictionFromAnAuthorizedGDB => 106,
            Self::AuthorizationDeenabled => 107,
            Self::AuthenticationRejectedDueToFILSAuthenticationFailure => 112,
            Self::AuthenticationRejectedDueToUnknownAuthenticationServer => 113,

            Self::Reserved(other) => other,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReasonCode {
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
    RequestedSTALeavingBSS,
    /// 37 Requested from peer STA as it does not want to use the mechanism
    RequestedSTADoesNotWantToUseTheMechanism,
    /// 38 Requested from peer STA as the STA received frames using the mechanism for which a setup is required
    RequestedSTAReceivedFrames,
    /// 39 Requested from peer STA due to timeout
    RequestedSTATimeout,
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

    Reserved(u16),
}
impl ReasonCode {
    #[must_use]
    pub fn from_u16(n: u16) -> Self {
        match n {
            1 => Self::UnspecifiedReason,
            2 => Self::PreviousAuthenticationNoLongerValid,
            3 => Self::STALeavingIBSSOrESS,
            4 => Self::Inactivity,
            5 => Self::APIsUnableToHandleAllCurrentlyAssociatedSTAs,
            6 => Self::Class2FrameReceivedFromNonauthenticatedSTA,
            7 => Self::Class3FrameReceivedFromNonassociatedSTA,
            8 => Self::STALeavingBSS,
            9 => Self::NotAuthenticated,
            10 => Self::PowerCapabilityElementIsUnacceptable,
            11 => Self::SupportedChannelsElementIsUnacceptable,
            12 => Self::DisassociatedDueToBSSTransitionManagement,
            13 => Self::InvalidInformationElement,
            14 => Self::MessageIntegrityCodeFailure,
            15 => Self::FourWayHandshakeTimeout,
            16 => Self::GroupKeyHandshakeTimeout,
            17 => Self::InformationDifferent,
            18 => Self::InvalidGroupCipher,
            19 => Self::InvalidPairwiseCipher,
            20 => Self::InvalidAKMP,
            21 => Self::UnsupportedRSNInformationElementVersion,
            22 => Self::InvalidRSNInformationElementCapabilities,
            23 => Self::IEEE8021XAuthenticationFailed,
            24 => Self::CipherSuiteRejectedBecauseOfTheSecurityPolicy,
            25 => Self::TDLSDirectLinkTeardownUnreachable,
            26 => Self::TDLSDirectLinkTeardownUnspecifiedReason,
            27 => Self::DisassociatedBecauseSessionTerminatedBySSPRequest,
            28 => Self::DisassociatedBecauseOfLackOfSSPRoamingAgreement,
            29 => Self::RequestedServiceRejected,
            30 => Self::RequestedServiceNotAuthorizedInThisLocation,
            31 => Self::TSDeleted,
            32 => Self::DisassociatedForUnspecifiedQoSRelatedReason,
            33 => Self::QoSAPLacksSufficientBandwidthForThisQoSSTA,
            34 => Self::ExcessiveNumberOfFramesNotAcknowledged,
            35 => Self::STAIsTransmittingOutsideTheLimitsOfItsTXOPs,
            36 => Self::RequestedSTALeavingBSS,
            37 => Self::RequestedSTADoesNotWantToUseTheMechanism,
            38 => Self::RequestedSTAReceivedFrames,
            39 => Self::RequestedSTATimeout,
            45 => Self::PeerSTADoesNotSupportTheRequestedCipherSuite,
            46 => Self::AuthorizedAccessLimitReached,
            47 => Self::DisassociatedDueToExternalServiceRequirements,
            48 => Self::InvalidFTActionFrameCount,
            49 => Self::InvalidPairwiseMasterKeyIdentifier,
            50 => Self::InvalidMDE,
            51 => Self::InvalidFTE,
            52 => Self::SMECancel,
            53 => Self::MeshMaxSupportedMaximumNumberOfPeerMeshSTAs,
            54 => Self::InformationViolatesMeshPolicy,
            55 => Self::MeshSTAReceivedAMeshPeeringCloseMessage,
            56 => Self::MeshSTAHasResentMaxRetriesWithoutConfirm,
            57 => Self::TheConfirmTimerForTheMeshPeeringInstanceTimesOut,
            58 => Self::MeshSTAFailsToUnwrapGTK,
            59 => Self::MeshSTAReceivesInconsistentInformation,
            60 => Self::MeshSTAFailsAuthenticatedMeshPeeringExchange,
            61 => Self::MeshSTADoesNotHaveProxyInformation,
            62 => Self::MeshSTADoesNotHaveForwardingInformation,
            63 => Self::MeshSTALinkNoLongerUsable,
            64 => Self::MacAddressAlreadyExists,
            65 => Self::MeshSTAPerformsChannelSwitchToMeetRegulatoryRequirements,
            66 => Self::MeshSTAPerformsChannelSwitchWithUnspecifiedReason,

            other => Self::Reserved(other),
        }
    }

    #[must_use]
    pub fn into_u16(self) -> u16 {
        match self {
            Self::UnspecifiedReason => 1,
            Self::PreviousAuthenticationNoLongerValid => 2,
            Self::STALeavingIBSSOrESS => 3,
            Self::Inactivity => 4,
            Self::APIsUnableToHandleAllCurrentlyAssociatedSTAs => 5,
            Self::Class2FrameReceivedFromNonauthenticatedSTA => 6,
            Self::Class3FrameReceivedFromNonassociatedSTA => 7,
            Self::STALeavingBSS => 8,
            Self::NotAuthenticated => 9,
            Self::PowerCapabilityElementIsUnacceptable => 10,
            Self::SupportedChannelsElementIsUnacceptable => 11,
            Self::DisassociatedDueToBSSTransitionManagement => 12,
            Self::InvalidInformationElement => 13,
            Self::MessageIntegrityCodeFailure => 14,
            Self::FourWayHandshakeTimeout => 15,
            Self::GroupKeyHandshakeTimeout => 16,
            Self::InformationDifferent => 17,
            Self::InvalidGroupCipher => 18,
            Self::InvalidPairwiseCipher => 19,
            Self::InvalidAKMP => 20,
            Self::UnsupportedRSNInformationElementVersion => 21,
            Self::InvalidRSNInformationElementCapabilities => 22,
            Self::IEEE8021XAuthenticationFailed => 23,
            Self::CipherSuiteRejectedBecauseOfTheSecurityPolicy => 24,
            Self::TDLSDirectLinkTeardownUnreachable => 25,
            Self::TDLSDirectLinkTeardownUnspecifiedReason => 26,
            Self::DisassociatedBecauseSessionTerminatedBySSPRequest => 27,
            Self::DisassociatedBecauseOfLackOfSSPRoamingAgreement => 28,
            Self::RequestedServiceRejected => 29,
            Self::RequestedServiceNotAuthorizedInThisLocation => 30,
            Self::TSDeleted => 31,
            Self::DisassociatedForUnspecifiedQoSRelatedReason => 32,
            Self::QoSAPLacksSufficientBandwidthForThisQoSSTA => 33,
            Self::ExcessiveNumberOfFramesNotAcknowledged => 34,
            Self::STAIsTransmittingOutsideTheLimitsOfItsTXOPs => 35,
            Self::RequestedSTALeavingBSS => 36,
            Self::RequestedSTADoesNotWantToUseTheMechanism => 37,
            Self::RequestedSTAReceivedFrames => 38,
            Self::RequestedSTATimeout => 39,
            Self::PeerSTADoesNotSupportTheRequestedCipherSuite => 45,
            Self::AuthorizedAccessLimitReached => 46,
            Self::DisassociatedDueToExternalServiceRequirements => 47,
            Self::InvalidFTActionFrameCount => 48,
            Self::InvalidPairwiseMasterKeyIdentifier => 49,
            Self::InvalidMDE => 50,
            Self::InvalidFTE => 51,
            Self::SMECancel => 52,
            Self::MeshMaxSupportedMaximumNumberOfPeerMeshSTAs => 53,
            Self::InformationViolatesMeshPolicy => 54,
            Self::MeshSTAReceivedAMeshPeeringCloseMessage => 55,
            Self::MeshSTAHasResentMaxRetriesWithoutConfirm => 56,
            Self::TheConfirmTimerForTheMeshPeeringInstanceTimesOut => 57,
            Self::MeshSTAFailsToUnwrapGTK => 58,
            Self::MeshSTAReceivesInconsistentInformation => 59,
            Self::MeshSTAFailsAuthenticatedMeshPeeringExchange => 60,
            Self::MeshSTADoesNotHaveProxyInformation => 61,
            Self::MeshSTADoesNotHaveForwardingInformation => 62,
            Self::MeshSTALinkNoLongerUsable => 63,
            Self::MacAddressAlreadyExists => 64,
            Self::MeshSTAPerformsChannelSwitchToMeetRegulatoryRequirements => 65,
            Self::MeshSTAPerformsChannelSwitchWithUnspecifiedReason => 66,

            Self::Reserved(other) => other,
        }
    }
}
