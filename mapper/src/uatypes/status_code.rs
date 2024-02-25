use crate::encoding_prelude::*;

#[derive(Debug)]
pub struct StatusCode(u32);

#[allow(non_upper_case_globals)]
//we conserve the same syntax as provided in the documentation
//the list of status code is extracted from : https://github.com/locka99/opcua/blob/master/lib/src/types/status_codes.rs
#[allow(dead_code)]
impl StatusCode {
    pub fn new(statuscode: u32) -> StatusCode {
       StatusCode(statuscode)
    }
    pub fn get_value(&self)-> u32{
        return self.0;
    }
    pub const Good: u32 = 0x00000000; //: u32="The operation succeeded."
    pub const Uncertain: u32 = 0x40000000; //: u32="The operation was uncertain."
    pub const Bad: u32 = 0x80000000; //: u32="The operation failed."
    pub const BadUnexpectedError: u32 = 0x80010000; //: u32="An unexpected error occurred."
    pub const BadInternalError: u32 = 0x80020000; //: u32="An internal error occurred as a result of a programming or configuration error."
    pub const BadOutOfMemory: u32 = 0x80030000; //: u32="Not enough memory to complete the operation."
    pub const BadResourceUnavailable: u32 = 0x80040000; //: u32="An operating system resource is not available."
    pub const BadCommunicationError: u32 = 0x80050000; //: u32="A low level communication error occurred."
    pub const BadEncodingError: u32 = 0x80060000; //: u32="Encoding halted because of invalid data in the objects being serialized."
    pub const BadDecodingError: u32 = 0x80070000; //: u32="Decoding halted because of invalid data in the stream."
    pub const BadEncodingLimitsExceeded: u32 = 0x80080000; //: u32="The message encoding/decoding limits imposed by the stack have been exceeded."
    pub const BadRequestTooLarge: u32 = 0x80B80000; //: u32="The request message size exceeds limits set by the server."
    pub const BadResponseTooLarge: u32 = 0x80B90000; //: u32="The response message size exceeds limits set by the client."
    pub const BadUnknownResponse: u32 = 0x80090000; //: u32="An unrecognized response was received from the server."
    pub const BadTimeout: u32 = 0x800A0000; //: u32="The operation timed out."
    pub const BadServiceUnsupported: u32 = 0x800B0000; //: u32="The server does not support the requested service."
    pub const BadShutdown: u32 = 0x800C0000; //: u32="The operation was cancelled because the application is shutting down."
    pub const BadServerNotConnected: u32 = 0x800D0000; //: u32="The operation could not complete because the client is not connected to the server."
    pub const BadServerHalted: u32 = 0x800E0000; //: u32="The server has stopped and cannot process any requests."
    pub const BadNothingToDo: u32 = 0x800F0000; //: u32="No processing could be done because there was nothing to do."
    pub const BadTooManyOperations: u32 = 0x80100000; //: u32="The request could not be processed because it specified too many operations."
    pub const BadTooManyMonitoredItems: u32 = 0x80DB0000; //: u32="The request could not be processed because there are too many monitored items in the subscription."
    pub const BadDataTypeIdUnknown: u32 = 0x80110000; //: u32="The extension object cannot be (de)serialized because the data type id is not recognized."
    pub const BadCertificateInvalid: u32 = 0x80120000; //: u32="The certificate provided as a parameter is not valid."
    pub const BadSecurityChecksFailed: u32 = 0x80130000; //: u32="An error occurred verifying security."
    pub const BadCertificatePolicyCheckFailed: u32 = 0x81140000; //: u32="The certificate does not meet the requirements of the security policy."
    pub const BadCertificateTimeInvalid: u32 = 0x80140000; //: u32="The certificate has expired or is not yet valid."
    pub const BadCertificateIssuerTimeInvalid: u32 = 0x80150000; //: u32="An issuer certificate has expired or is not yet valid."
    pub const BadCertificateHostNameInvalid: u32 = 0x80160000; //: u32="The HostName used to connect to a server does not match a HostName in the certificate."
    pub const BadCertificateUriInvalid: u32 = 0x80170000; //: u32="The URI specified in the ApplicationDescription does not match the URI in the certificate."
    pub const BadCertificateUseNotAllowed: u32 = 0x80180000; //: u32="The certificate may not be used for the requested operation."
    pub const BadCertificateIssuerUseNotAllowed: u32 = 0x80190000; //: u32="The issuer certificate may not be used for the requested operation."
    pub const BadCertificateUntrusted: u32 = 0x801A0000; //: u32="The certificate is not trusted."
    pub const BadCertificateRevocationUnknown: u32 = 0x801B0000; //: u32="It was not possible to determine if the certificate has been revoked."
    pub const BadCertificateIssuerRevocationUnknown: u32 = 0x801C0000; //: u32="It was not possible to determine if the issuer certificate has been revoked."
    pub const BadCertificateRevoked: u32 = 0x801D0000; //: u32="The certificate has been revoked."
    pub const BadCertificateIssuerRevoked: u32 = 0x801E0000; //: u32="The issuer certificate has been revoked."
    pub const BadCertificateChainIncomplete: u32 = 0x810D0000; //: u32="The certificate chain is incomplete."
    pub const BadUserAccessDenied: u32 = 0x801F0000; //: u32="User does not have permission to perform the requested operation."
    pub const BadIdentityTokenInvalid: u32 = 0x80200000; //: u32="The user identity token is not valid."
    pub const BadIdentityTokenRejected: u32 = 0x80210000; //: u32="The user identity token is valid but the server has rejected it."
    pub const BadSecureChannelIdInvalid: u32 = 0x80220000; //: u32="The specified secure channel is no longer valid."
    pub const BadInvalidTimestamp: u32 = 0x80230000; //: u32="The timestamp is outside the range allowed by the server."
    pub const BadNonceInvalid: u32 = 0x80240000; //: u32="The nonce does appear to be not a random value or it is not the correct length."
    pub const BadSessionIdInvalid: u32 = 0x80250000; //: u32="The session id is not valid."
    pub const BadSessionClosed: u32 = 0x80260000; //: u32="The session was closed by the client."
    pub const BadSessionNotActivated: u32 = 0x80270000; //: u32="The session cannot be used because ActivateSession has not been called."
    pub const BadSubscriptionIdInvalid: u32 = 0x80280000; //: u32="The subscription id is not valid."
    pub const BadRequestHeaderInvalid: u32 = 0x802A0000; //: u32="The header for the request is missing or invalid."
    pub const BadTimestampsToReturnInvalid: u32 = 0x802B0000; //: u32="The timestamps to return parameter is invalid."
    pub const BadRequestCancelledByClient: u32 = 0x802C0000; //: u32="The request was cancelled by the client."
    pub const BadTooManyArguments: u32 = 0x80E50000; //: u32="Too many arguments were provided."
    pub const BadLicenseExpired: u32 = 0x810E0000; //: u32="The server requires a license to operate in general or to perform a service or operation: u32= but existing license is expired."
    pub const BadLicenseLimitsExceeded: u32 = 0x810F0000; //: u32="The server has limits on number of allowed operations / objects: u32= based on installed licenses: u32= and these limits where exceeded."
    pub const BadLicenseNotAvailable: u32 = 0x81100000; //: u32="The server does not have a license which is required to operate in general or to perform a service or operation."
    pub const GoodSubscriptionTransferred: u32 = 0x002D0000; //: u32="The subscription was transferred to another session."
    pub const GoodCompletesAsynchronously: u32 = 0x002E0000; //: u32="The processing will complete asynchronously."
    pub const GoodOverload: u32 = 0x002F0000; //: u32="Sampling has slowed down due to resource limitations."
    pub const GoodClamped: u32 = 0x00300000; //: u32="The value written was accepted but was clamped."
    pub const BadNoCommunication: u32 = 0x80310000; //: u32="Communication with the data source is defined: u32= but not established: u32= and there is no last known value available."
    pub const BadWaitingForInitialData: u32 = 0x80320000; //: u32="Waiting for the server to obtain values from the underlying data source."
    pub const BadNodeIdInvalid: u32 = 0x80330000; //: u32="The syntax of the node id is not valid."
    pub const BadNodeIdUnknown: u32 = 0x80340000; //: u32="The node id refers to a node that does not exist in the server address space."
    pub const BadAttributeIdInvalid: u32 = 0x80350000; //: u32="The attribute is not supported for the specified Node."
    pub const BadIndexRangeInvalid: u32 = 0x80360000; //: u32="The syntax of the index range parameter is invalid."
    pub const BadIndexRangeNoData: u32 = 0x80370000; //: u32="No data exists within the range of indexes specified."
    pub const BadDataEncodingInvalid: u32 = 0x80380000; //: u32="The data encoding is invalid."
    pub const BadDataEncodingUnsupported: u32 = 0x80390000; //: u32="The server does not support the requested data encoding for the node."
    pub const BadNotReadable: u32 = 0x803A0000; //: u32="The access level does not allow reading or subscribing to the Node."
    pub const BadNotWritable: u32 = 0x803B0000; //: u32="The access level does not allow writing to the Node."
    pub const BadOutOfRange: u32 = 0x803C0000; //: u32="The value was out of range."
    pub const BadNotSupported: u32 = 0x803D0000; //: u32="The requested operation is not supported."
    pub const BadNotFound: u32 = 0x803E0000; //: u32="A requested item was not found or a search operation ended without success."
    pub const BadObjectDeleted: u32 = 0x803F0000; //: u32="The object cannot be used because it has been deleted."
    pub const BadNotImplemented: u32 = 0x80400000; //: u32="Requested operation is not implemented."
    pub const BadMonitoringModeInvalid: u32 = 0x80410000; //: u32="The monitoring mode is invalid."
    pub const BadMonitoredItemIdInvalid: u32 = 0x80420000; //: u32="The monitoring item id does not refer to a valid monitored item."
    pub const BadMonitoredItemFilterInvalid: u32 = 0x80430000; //: u32="The monitored item filter parameter is not valid."
    pub const BadMonitoredItemFilterUnsupported: u32 = 0x80440000; //: u32="The server does not support the requested monitored item filter."
    pub const BadFilterNotAllowed: u32 = 0x80450000; //: u32="A monitoring filter cannot be used in combination with the attribute specified."
    pub const BadStructureMissing: u32 = 0x80460000; //: u32="A mandatory structured parameter was missing or null."
    pub const BadEventFilterInvalid: u32 = 0x80470000; //: u32="The event filter is not valid."
    pub const BadContentFilterInvalid: u32 = 0x80480000; //: u32="The content filter is not valid."
    pub const BadFilterOperatorInvalid: u32 = 0x80C10000; //: u32="An unrecognized operator was provided in a filter."
    pub const BadFilterOperatorUnsupported: u32 = 0x80C20000; //: u32="A valid operator was provided: u32= but the server does not provide support for this filter operator."
    pub const BadFilterOperandCountMismatch: u32 = 0x80C30000; //: u32="The number of operands provided for the filter operator was less then expected for the operand provided."
    pub const BadFilterOperandInvalid: u32 = 0x80490000; //: u32="The operand used in a content filter is not valid."
    pub const BadFilterElementInvalid: u32 = 0x80C40000; //: u32="The referenced element is not a valid element in the content filter."
    pub const BadFilterLiteralInvalid: u32 = 0x80C50000; //: u32="The referenced literal is not a valid value."
    pub const BadContinuationPointInvalid: u32 = 0x804A0000; //: u32="The continuation point provide is longer valid."
    pub const BadNoContinuationPoints: u32 = 0x804B0000; //: u32="The operation could not be processed because all continuation points have been allocated."
    pub const BadReferenceTypeIdInvalid: u32 = 0x804C0000; //: u32="The reference type id does not refer to a valid reference type node."
    pub const BadBrowseDirectionInvalid: u32 = 0x804D0000; //: u32="The browse direction is not valid."
    pub const BadNodeNotInView: u32 = 0x804E0000; //: u32="The node is not part of the view."
    pub const BadNumericOverflow: u32 = 0x81120000; //: u32="The number was not accepted because of a numeric overflow."
    pub const BadServerUriInvalid: u32 = 0x804F0000; //: u32="The ServerUri is not a valid URI."
    pub const BadServerNameMissing: u32 = 0x80500000; //: u32="No ServerName was specified."
    pub const BadDiscoveryUrlMissing: u32 = 0x80510000; //: u32="No DiscoveryUrl was specified."
    pub const BadSempahoreFileMissing: u32 = 0x80520000; //: u32="The semaphore file specified by the client is not valid."
    pub const BadRequestTypeInvalid: u32 = 0x80530000; //: u32="The security token request type is not valid."
    pub const BadSecurityModeRejected: u32 = 0x80540000; //: u32="The security mode does not meet the requirements set by the server."
    pub const BadSecurityPolicyRejected: u32 = 0x80550000; //: u32="The security policy does not meet the requirements set by the server."
    pub const BadTooManySessions: u32 = 0x80560000; //: u32="The server has reached its maximum number of sessions."
    pub const BadUserSignatureInvalid: u32 = 0x80570000; //: u32="The user token signature is missing or invalid."
    pub const BadApplicationSignatureInvalid: u32 = 0x80580000; //: u32="The signature generated with the client certificate is missing or invalid."
    pub const BadNoValidCertificates: u32 = 0x80590000; //: u32="The client did not provide at least one software certificate that is valid and meets the profile requirements for the server."
    pub const BadIdentityChangeNotSupported: u32 = 0x80C60000; //: u32="The server does not support changing the user identity assigned to the session."
    pub const BadRequestCancelledByRequest: u32 = 0x805A0000; //: u32="The request was cancelled by the client with the Cancel service."
    pub const BadParentNodeIdInvalid: u32 = 0x805B0000; //: u32="The parent node id does not to refer to a valid node."
    pub const BadReferenceNotAllowed: u32 = 0x805C0000; //: u32="The reference could not be created because it violates constraints imposed by the data model."
    pub const BadNodeIdRejected: u32 = 0x805D0000; //: u32="The requested node id was reject because it was either invalid or server does not allow node ids to be specified by the client."
    pub const BadNodeIdExists: u32 = 0x805E0000; //: u32="The requested node id is already used by another node."
    pub const BadNodeClassInvalid: u32 = 0x805F0000; //: u32="The node class is not valid."
    pub const BadBrowseNameInvalid: u32 = 0x80600000; //: u32="The browse name is invalid."
    pub const BadBrowseNameDuplicated: u32 = 0x80610000; //: u32="The browse name is not unique among nodes that share the same relationship with the parent."
    pub const BadNodeAttributesInvalid: u32 = 0x80620000; //: u32="The node attributes are not valid for the node class."
    pub const BadTypeDefinitionInvalid: u32 = 0x80630000; //: u32="The type definition node id does not reference an appropriate type node."
    pub const BadSourceNodeIdInvalid: u32 = 0x80640000; //: u32="The source node id does not reference a valid node."
    pub const BadTargetNodeIdInvalid: u32 = 0x80650000; //: u32="The target node id does not reference a valid node."
    pub const BadDuplicateReferenceNotAllowed: u32 = 0x80660000; //: u32="The reference type between the nodes is already defined."
    pub const BadInvalidSelfReference: u32 = 0x80670000; //: u32="The server does not allow this type of self reference on this node."
    pub const BadReferenceLocalOnly: u32 = 0x80680000; //: u32="The reference type is not valid for a reference to a remote server."
    pub const BadNoDeleteRights: u32 = 0x80690000; //: u32="The server will not allow the node to be deleted."
    pub const UncertainReferenceNotDeleted: u32 = 0x40BC0000; //: u32="The server was not able to delete all target references."
    pub const BadServerIndexInvalid: u32 = 0x806A0000; //: u32="The server index is not valid."
    pub const BadViewIdUnknown: u32 = 0x806B0000; //: u32="The view id does not refer to a valid view node."
    pub const BadViewTimestampInvalid: u32 = 0x80C90000; //: u32="The view timestamp is not available or not supported."
    pub const BadViewParameterMismatch: u32 = 0x80CA0000; //: u32="The view parameters are not consistent with each other."
    pub const BadViewVersionInvalid: u32 = 0x80CB0000; //: u32="The view version is not available or not supported."
    pub const UncertainNotAllNodesAvailable: u32 = 0x40C00000; //: u32="The list of references may not be complete because the underlying system is not available."
    pub const GoodResultsMayBeIncomplete: u32 = 0x00BA0000; //: u32="The server should have followed a reference to a node in a remote server but did not. The result set may be incomplete."
    pub const BadNotTypeDefinition: u32 = 0x80C80000; //: u32="The provided Nodeid was not a type definition nodeid."
    pub const UncertainReferenceOutOfServer: u32 = 0x406C0000; //: u32="One of the references to follow in the relative path references to a node in the address space in another server."
    pub const BadTooManyMatches: u32 = 0x806D0000; //: u32="The requested operation has too many matches to return."
    pub const BadQueryTooComplex: u32 = 0x806E0000; //: u32="The requested operation requires too many resources in the server."
    pub const BadNoMatch: u32 = 0x806F0000; //: u32="The requested operation has no match to return."
    pub const BadMaxAgeInvalid: u32 = 0x80700000; //: u32="The max age parameter is invalid."
    pub const BadSecurityModeInsufficient: u32 = 0x80E60000; //: u32="The operation is not permitted over the current secure channel."
    pub const BadHistoryOperationInvalid: u32 = 0x80710000; //: u32="The history details parameter is not valid."
    pub const BadHistoryOperationUnsupported: u32 = 0x80720000; //: u32="The server does not support the requested operation."
    pub const BadInvalidTimestampArgument: u32 = 0x80BD0000; //: u32="The defined timestamp to return was invalid."
    pub const BadWriteNotSupported: u32 = 0x80730000; //: u32="The server does not support writing the combination of value: u32= status and timestamps provided."
    pub const BadTypeMismatch: u32 = 0x80740000; //: u32="The value supplied for the attribute is not of the same type as the attribute's value."
    pub const BadMethodInvalid: u32 = 0x80750000; //: u32="The method id does not refer to a method for the specified object."
    pub const BadArgumentsMissing: u32 = 0x80760000; //: u32="The client did not specify all of the input arguments for the method."
    pub const BadNotExecutable: u32 = 0x81110000; //: u32="The executable attribute does not allow the execution of the method."
    pub const BadTooManySubscriptions: u32 = 0x80770000; //: u32="The server has reached its maximum number of subscriptions."
    pub const BadTooManyPublishRequests: u32 = 0x80780000; //: u32="The server has reached the maximum number of queued publish requests."
    pub const BadNoSubscription: u32 = 0x80790000; //: u32="There is no subscription available for this session."
    pub const BadSequenceNumberUnknown: u32 = 0x807A0000; //: u32="The sequence number is unknown to the server."
    pub const GoodRetransmissionQueueNotSupported: u32 = 0x00DF0000; //: u32="The Server does not support retransmission queue and acknowledgement of sequence numbers is not available."
    pub const BadMessageNotAvailable: u32 = 0x807B0000; //: u32="The requested notification message is no longer available."
    pub const BadInsufficientClientProfile: u32 = 0x807C0000; //: u32="The client of the current session does not support one or more Profiles that are necessary for the subscription."
    pub const BadStateNotActive: u32 = 0x80BF0000; //: u32="The sub-state machine is not currently active."
    pub const BadAlreadyExists: u32 = 0x81150000; //: u32="An equivalent rule already exists."
    pub const BadTcpServerTooBusy: u32 = 0x807D0000; //: u32="The server cannot process the request because it is too busy."
    pub const BadTcpMessageTypeInvalid: u32 = 0x807E0000; //: u32="The type of the message specified in the header invalid."
    pub const BadTcpSecureChannelUnknown: u32 = 0x807F0000; //: u32="The SecureChannelId and/or TokenId are not currently in use."
    pub const BadTcpMessageTooLarge: u32 = 0x80800000; //: u32="The size of the message chunk specified in the header is too large."
    pub const BadTcpNotEnoughResources: u32 = 0x80810000; //: u32="There are not enough resources to process the request."
    pub const BadTcpInternalError: u32 = 0x80820000; //: u32="An internal error occurred."
    pub const BadTcpEndpointUrlInvalid: u32 = 0x80830000; //: u32="The server does not recognize the QueryString specified."
    pub const BadRequestInterrupted: u32 = 0x80840000; //: u32="The request could not be sent because of a network interruption."
    pub const BadRequestTimeout: u32 = 0x80850000; //: u32="Timeout occurred while processing the request."
    pub const BadSecureChannelClosed: u32 = 0x80860000; //: u32="The secure channel has been closed."
    pub const BadSecureChannelTokenUnknown: u32 = 0x80870000; //: u32="The token has expired or is not recognized."
    pub const BadSequenceNumberInvalid: u32 = 0x80880000; //: u32="The sequence number is not valid."
    pub const BadProtocolVersionUnsupported: u32 = 0x80BE0000; //: u32="The applications do not have compatible protocol versions."
    pub const BadConfigurationError: u32 = 0x80890000; //: u32="There is a problem with the configuration that affects the usefulness of the value."
    pub const BadNotConnected: u32 = 0x808A0000; //: u32="The variable should receive its value from another variable: u32= but has never been configured to do so."
    pub const BadDeviceFailure: u32 = 0x808B0000; //: u32="There has been a failure in the device/data source that generates the value that has affected the value."
    pub const BadSensorFailure: u32 = 0x808C0000; //: u32="There has been a failure in the sensor from which the value is derived by the device/data source."
    pub const BadOutOfService: u32 = 0x808D0000; //: u32="The source of the data is not operational."
    pub const BadDeadbandFilterInvalid: u32 = 0x808E0000; //: u32="The deadband filter is not valid."
    pub const UncertainNoCommunicationLastUsableValue: u32 = 0x408F0000; //: u32="Communication to the data source has failed. The variable value is the last value that had a good quality."
    pub const UncertainLastUsableValue: u32 = 0x40900000; //: u32="Whatever was updating this value has stopped doing so."
    pub const UncertainSubstituteValue: u32 = 0x40910000; //: u32="The value is an operational value that was manually overwritten."
    pub const UncertainInitialValue: u32 = 0x40920000; //: u32="The value is an initial value for a variable that normally receives its value from another variable."
    pub const UncertainSensorNotAccurate: u32 = 0x40930000; //: u32="The value is at one of the sensor limits."
    pub const UncertainEngineeringUnitsExceeded: u32 = 0x40940000; //: u32="The value is outside of the range of values defined for this parameter."
    pub const UncertainSubNormal: u32 = 0x40950000; //: u32="The value is derived from multiple sources and has less than the required number of Good sources."
    pub const GoodLocalOverride: u32 = 0x00960000; //: u32="The value has been overridden."
    pub const BadRefreshInProgress: u32 = 0x80970000; //: u32="This Condition refresh failed: u32= a Condition refresh operation is already in progress."
    pub const BadConditionAlreadyDisabled: u32 = 0x80980000; //: u32="This condition has already been disabled."
    pub const BadConditionAlreadyEnabled: u32 = 0x80CC0000; //: u32="This condition has already been enabled."
    pub const BadConditionDisabled: u32 = 0x80990000; //: u32="Property not available: u32= this condition is disabled."
    pub const BadEventIdUnknown: u32 = 0x809A0000; //: u32="The specified event id is not recognized."
    pub const BadEventNotAcknowledgeable: u32 = 0x80BB0000; //: u32="The event cannot be acknowledged."
    pub const BadDialogNotActive: u32 = 0x80CD0000; //: u32="The dialog condition is not active."
    pub const BadDialogResponseInvalid: u32 = 0x80CE0000; //: u32="The response is not valid for the dialog."
    pub const BadConditionBranchAlreadyAcked: u32 = 0x80CF0000; //: u32="The condition branch has already been acknowledged."
    pub const BadConditionBranchAlreadyConfirmed: u32 = 0x80D00000; //: u32="The condition branch has already been confirmed."
    pub const BadConditionAlreadyShelved: u32 = 0x80D10000; //: u32="The condition has already been shelved."
    pub const BadConditionNotShelved: u32 = 0x80D20000; //: u32="The condition is not currently shelved."
    pub const BadShelvingTimeOutOfRange: u32 = 0x80D30000; //: u32="The shelving time not within an acceptable range."
    pub const BadNoData: u32 = 0x809B0000; //: u32="No data exists for the requested time range or event filter."
    pub const BadBoundNotFound: u32 = 0x80D70000; //: u32="No data found to provide upper or lower bound value."
    pub const BadBoundNotSupported: u32 = 0x80D80000; //: u32="The server cannot retrieve a bound for the variable."
    pub const BadDataLost: u32 = 0x809D0000; //: u32="Data is missing due to collection started/stopped/lost."
    pub const BadDataUnavailable: u32 = 0x809E0000; //: u32="Expected data is unavailable for the requested time range due to an un-mounted volume: u32= an off-line archive or tape: u32= or similar reason for temporary unavailability."
    pub const BadEntryExists: u32 = 0x809F0000; //: u32="The data or event was not successfully inserted because a matching entry exists."
    pub const BadNoEntryExists: u32 = 0x80A00000; //: u32="The data or event was not successfully updated because no matching entry exists."
    pub const BadTimestampNotSupported: u32 = 0x80A10000; //: u32="The client requested history using a timestamp format the server does not support (i.e requested ServerTimestamp when server only supports SourceTimestamp)."
    pub const GoodEntryInserted: u32 = 0x00A20000; //: u32="The data or event was successfully inserted into the historical database."
    pub const GoodEntryReplaced: u32 = 0x00A30000; //: u32="The data or event field was successfully replaced in the historical database."
    pub const UncertainDataSubNormal: u32 = 0x40A40000; //: u32="The value is derived from multiple values and has less than the required number of Good values."
    pub const GoodNoData: u32 = 0x00A50000; //: u32="No data exists for the requested time range or event filter."
    pub const GoodMoreData: u32 = 0x00A60000; //: u32="The data or event field was successfully replaced in the historical database."
    pub const BadAggregateListMismatch: u32 = 0x80D40000; //: u32="The requested number of Aggregates does not match the requested number of NodeIds."
    pub const BadAggregateNotSupported: u32 = 0x80D50000; //: u32="The requested Aggregate is not support by the server."
    pub const BadAggregateInvalidInputs: u32 = 0x80D60000; //: u32="The aggregate value could not be derived due to invalid data inputs."
    pub const BadAggregateConfigurationRejected: u32 = 0x80DA0000; //: u32="The aggregate configuration is not valid for specified node."
    pub const GoodDataIgnored: u32 = 0x00D90000; //: u32="The request specifies fields which are not valid for the EventType or cannot be saved by the historian."
    pub const BadRequestNotAllowed: u32 = 0x80E40000; //: u32="The request was rejected by the server because it did not meet the criteria set by the server."
    pub const BadRequestNotComplete: u32 = 0x81130000; //: u32="The request has not been processed by the server yet."
    pub const BadTicketRequired: u32 = 0x811F0000; //: u32="The device identity needs a ticket before it can be accepted."
    pub const BadTicketInvalid: u32 = 0x81200000; //: u32="The device identity needs a ticket before it can be accepted."
    pub const GoodEdited: u32 = 0x00DC0000; //: u32="The value does not come from the real source and has been edited by the server."
    pub const GoodPostActionFailed: u32 = 0x00DD0000; //: u32="There was an error in execution of these post-actions."
    pub const UncertainDominantValueChanged: u32 = 0x40DE0000; //: u32="The related EngineeringUnit has been changed but the Variable Value is still provided based on the previous unit."
    pub const GoodDependentValueChanged: u32 = 0x00E00000; //: u32="A dependent value has been changed but the change has not been applied to the device."
    pub const BadDominantValueChanged: u32 = 0x80E10000; //: u32="The related EngineeringUnit has been changed but this change has not been applied to the device. The Variable Value is still dependent on the previous unit but its status is currently Bad."
    pub const UncertainDependentValueChanged: u32 = 0x40E20000; //: u32="A dependent value has been changed but the change has not been applied to the device. The quality of the dominant variable is uncertain."
    pub const BadDependentValueChanged: u32 = 0x80E30000; //: u32="A dependent value has been changed but the change has not been applied to the device. The quality of the dominant variable is Bad."
    pub const GoodEdited_DependentValueChanged: u32 = 0x01160000; //: u32="It is delivered with a dominant Variable value when a dependent Variable has changed but the change has not been applied."
    pub const GoodEdited_DominantValueChanged: u32 = 0x01170000; //: u32="It is delivered with a dependent Variable value when a dominant Variable has changed but the change has not been applied."
    pub const GoodEdited_DominantValueChanged_DependentValueChanged: u32 = 0x01180000; //: u32="It is delivered with a dependent Variable value when a dominant or dependent Variable has changed but change has not been applied."
    pub const BadEdited_OutOfRange: u32 = 0x81190000; //: u32="It is delivered with a Variable value when Variable has changed but the value is not legal."
    pub const BadInitialValue_OutOfRange: u32 = 0x811A0000; //: u32="It is delivered with a Variable value when a source Variable has changed but the value is not legal."
    pub const BadOutOfRange_DominantValueChanged: u32 = 0x811B0000; //: u32="It is delivered with a dependent Variable value when a dominant Variable has changed and the value is not legal."
    pub const BadEdited_OutOfRange_DominantValueChanged: u32 = 0x811C0000; //: u32="It is delivered with a dependent Variable value when a dominant Variable has changed: u32= the value is not legal and the change has not been applied."
    pub const BadOutOfRange_DominantValueChanged_DependentValueChanged: u32 = 0x811D0000; //: u32="It is delivered with a dependent Variable value when a dominant or dependent Variable has changed and the value is not legal."
    pub const BadEdited_OutOfRange_DominantValueChanged_DependentValueChanged: u32 = 0x811E0000; //: u32="It is delivered with a dependent Variable value when a dominant or dependent Variable has changed: u32= the value is not legal and the change has not been applied."
    pub const GoodCommunicationEvent: u32 = 0x00A70000; //: u32="The communication layer has raised an event."
    pub const GoodShutdownEvent: u32 = 0x00A80000; //: u32="The system is shutting down."
    pub const GoodCallAgain: u32 = 0x00A90000; //: u32="The operation is not finished and needs to be called again."
    pub const GoodNonCriticalTimeout: u32 = 0x00AA0000; //: u32="A non-critical timeout occurred."
    pub const BadInvalidArgument: u32 = 0x80AB0000; //: u32="One or more arguments are invalid."
    pub const BadConnectionRejected: u32 = 0x80AC0000; //: u32="Could not establish a network connection to remote server."
    pub const BadDisconnect: u32 = 0x80AD0000; //: u32="The server has disconnected from the client."
    pub const BadConnectionClosed: u32 = 0x80AE0000; //: u32="The network connection has been closed."
    pub const BadInvalidState: u32 = 0x80AF0000; //: u32="The operation cannot be completed because the object is closed: u32= uninitialized or in some other invalid state."
    pub const BadEndOfStream: u32 = 0x80B00000; //: u32="Cannot move beyond end of the stream."
    pub const BadNoDataAvailable: u32 = 0x80B10000; //: u32="No data is currently available for reading from a non-blocking stream."
    pub const BadWaitingForResponse: u32 = 0x80B20000; //: u32="The asynchronous operation is waiting for a response."
    pub const BadOperationAbandoned: u32 = 0x80B30000; //: u32="The asynchronous operation was abandoned by the caller."
    pub const BadExpectedStreamToBlock: u32 = 0x80B40000; //: u32="The stream did not return all data requested (possibly because it is a non-blocking stream)."
    pub const BadWouldBlock: u32 = 0x80B50000; //: u32="Non blocking behaviour is required and the operation would block."
    pub const BadSyntaxError: u32 = 0x80B60000; //: u32="A value had an invalid syntax."
    pub const BadMaxConnectionsReached: u32 = 0x80B70000; //: u32="The operation could not be finished because all available connections are in use."
    pub const UncertainTransducerInManual: u32 = 0x42080000; //: u32="The value may not be accurate because the transducer is in manual mode."
    pub const UncertainSimulatedValue: u32 = 0x42090000; //: u32="The value is simulated."
    pub const UncertainSensorCalibration: u32 = 0x420A0000; //: u32="The value may not be accurate due to a sensor calibration fault."
    pub const UncertainConfigurationError: u32 = 0x420F0000; //: u32="The value may not be accurate due to a configuration issue."
    pub const GoodCascadeInitializationAcknowledged: u32 = 0x04010000; //: u32="The value source supports cascade handshaking and the value has been Initialized based on an initialization request from a cascade secondary."
    pub const GoodCascadeInitializationRequest: u32 = 0x04020000; //: u32="The value source supports cascade handshaking and is requesting initialization of a cascade primary."
    pub const GoodCascadeNotInvited: u32 = 0x04030000; //: u32="The value source supports cascade handshaking: u32= however: u32= the source's current state does not allow for cascade."
    pub const GoodCascadeNotSelected: u32 = 0x04040000; //: u32="The value source supports cascade handshaking: u32= however: u32= the source has not selected the corresponding cascade primary for use."
    pub const GoodFaultStateActive: u32 = 0x04070000; //: u32="There is a fault state condition active in the value source."
    pub const GoodInitiateFaultState: u32 = 0x04080000; //: u32="A fault state condition is being requested of the destination."
    pub const GoodCascade: u32 = 0x04090000; //: u32="The value is accurate: u32= and the signal source supports cascade handshaking."
    pub const BadDataSetIdInvalid: u32 = 0x80E70000; //: u32="The DataSet specified for the DataSetWriter creation is invalid."

    pub fn is_status_code(code: &u32) -> bool {
        match *code {
            StatusCode::Good => true,
            StatusCode::Uncertain => true,
            StatusCode::Bad => true,
            StatusCode::BadUnexpectedError => true,
            StatusCode::BadInternalError => true,
            StatusCode::BadOutOfMemory => true,
            StatusCode::BadResourceUnavailable => true,
            StatusCode::BadCommunicationError => true,
            StatusCode::BadEncodingError => true,
            StatusCode::BadDecodingError => true,
            StatusCode::BadEncodingLimitsExceeded => true,
            StatusCode::BadRequestTooLarge => true,
            StatusCode::BadResponseTooLarge => true,
            StatusCode::BadUnknownResponse => true,
            StatusCode::BadTimeout => true,
            StatusCode::BadServiceUnsupported => true,
            StatusCode::BadShutdown => true,
            StatusCode::BadServerNotConnected => true,
            StatusCode::BadServerHalted => true,
            StatusCode::BadNothingToDo => true,
            StatusCode::BadTooManyOperations => true,
            StatusCode::BadTooManyMonitoredItems => true,
            StatusCode::BadDataTypeIdUnknown => true,
            StatusCode::BadCertificateInvalid => true,
            StatusCode::BadSecurityChecksFailed => true,
            StatusCode::BadCertificatePolicyCheckFailed => true,
            StatusCode::BadCertificateTimeInvalid => true,
            StatusCode::BadCertificateIssuerTimeInvalid => true,
            StatusCode::BadCertificateHostNameInvalid => true,
            StatusCode::BadCertificateUriInvalid => true,
            StatusCode::BadCertificateUseNotAllowed => true,
            StatusCode::BadCertificateIssuerUseNotAllowed => true,
            StatusCode::BadCertificateUntrusted => true,
            StatusCode::BadCertificateRevocationUnknown => true,
            StatusCode::BadCertificateIssuerRevocationUnknown => true,
            StatusCode::BadCertificateRevoked => true,
            StatusCode::BadCertificateIssuerRevoked => true,
            StatusCode::BadCertificateChainIncomplete => true,
            StatusCode::BadUserAccessDenied => true,
            StatusCode::BadIdentityTokenInvalid => true,
            StatusCode::BadIdentityTokenRejected => true,
            StatusCode::BadSecureChannelIdInvalid => true,
            StatusCode::BadInvalidTimestamp => true,
            StatusCode::BadNonceInvalid => true,
            StatusCode::BadSessionIdInvalid => true,
            StatusCode::BadSessionClosed => true,
            StatusCode::BadSessionNotActivated => true,
            StatusCode::BadSubscriptionIdInvalid => true,
            StatusCode::BadRequestHeaderInvalid => true,
            StatusCode::BadTimestampsToReturnInvalid => true,
            StatusCode::BadRequestCancelledByClient => true,
            StatusCode::BadTooManyArguments => true,
            StatusCode::BadLicenseExpired => true,
            StatusCode::BadLicenseLimitsExceeded => true,
            StatusCode::BadLicenseNotAvailable => true,
            StatusCode::GoodSubscriptionTransferred => true,
            StatusCode::GoodCompletesAsynchronously => true,
            StatusCode::GoodOverload => true,
            StatusCode::GoodClamped => true,
            StatusCode::BadNoCommunication => true,
            StatusCode::BadWaitingForInitialData => true,
            StatusCode::BadNodeIdInvalid => true,
            StatusCode::BadNodeIdUnknown => true,
            StatusCode::BadAttributeIdInvalid => true,
            StatusCode::BadIndexRangeInvalid => true,
            StatusCode::BadIndexRangeNoData => true,
            StatusCode::BadDataEncodingInvalid => true,
            StatusCode::BadDataEncodingUnsupported => true,
            StatusCode::BadNotReadable => true,
            StatusCode::BadNotWritable => true,
            StatusCode::BadOutOfRange => true,
            StatusCode::BadNotSupported => true,
            StatusCode::BadNotFound => true,
            StatusCode::BadObjectDeleted => true,
            StatusCode::BadNotImplemented => true,
            StatusCode::BadMonitoringModeInvalid => true,
            StatusCode::BadMonitoredItemIdInvalid => true,
            StatusCode::BadMonitoredItemFilterInvalid => true,
            StatusCode::BadMonitoredItemFilterUnsupported => true,
            StatusCode::BadFilterNotAllowed => true,
            StatusCode::BadStructureMissing => true,
            StatusCode::BadEventFilterInvalid => true,
            StatusCode::BadContentFilterInvalid => true,
            StatusCode::BadFilterOperatorInvalid => true,
            StatusCode::BadFilterOperatorUnsupported => true,
            StatusCode::BadFilterOperandCountMismatch => true,
            StatusCode::BadFilterOperandInvalid => true,
            StatusCode::BadFilterElementInvalid => true,
            StatusCode::BadFilterLiteralInvalid => true,
            StatusCode::BadContinuationPointInvalid => true,
            StatusCode::BadNoContinuationPoints => true,
            StatusCode::BadReferenceTypeIdInvalid => true,
            StatusCode::BadBrowseDirectionInvalid => true,
            StatusCode::BadNodeNotInView => true,
            StatusCode::BadNumericOverflow => true,
            StatusCode::BadServerUriInvalid => true,
            StatusCode::BadServerNameMissing => true,
            StatusCode::BadDiscoveryUrlMissing => true,
            StatusCode::BadSempahoreFileMissing => true,
            StatusCode::BadRequestTypeInvalid => true,
            StatusCode::BadSecurityModeRejected => true,
            StatusCode::BadSecurityPolicyRejected => true,
            StatusCode::BadTooManySessions => true,
            StatusCode::BadUserSignatureInvalid => true,
            StatusCode::BadApplicationSignatureInvalid => true,
            StatusCode::BadNoValidCertificates => true,
            StatusCode::BadIdentityChangeNotSupported => true,
            StatusCode::BadRequestCancelledByRequest => true,
            StatusCode::BadParentNodeIdInvalid => true,
            StatusCode::BadReferenceNotAllowed => true,
            StatusCode::BadNodeIdRejected => true,
            StatusCode::BadNodeIdExists => true,
            StatusCode::BadNodeClassInvalid => true,
            StatusCode::BadBrowseNameInvalid => true,
            StatusCode::BadBrowseNameDuplicated => true,
            StatusCode::BadNodeAttributesInvalid => true,
            StatusCode::BadTypeDefinitionInvalid => true,
            StatusCode::BadSourceNodeIdInvalid => true,
            StatusCode::BadTargetNodeIdInvalid => true,
            StatusCode::BadDuplicateReferenceNotAllowed => true,
            StatusCode::BadInvalidSelfReference => true,
            StatusCode::BadReferenceLocalOnly => true,
            StatusCode::BadNoDeleteRights => true,
            StatusCode::UncertainReferenceNotDeleted => true,
            StatusCode::BadServerIndexInvalid => true,
            StatusCode::BadViewIdUnknown => true,
            StatusCode::BadViewTimestampInvalid => true,
            StatusCode::BadViewParameterMismatch => true,
            StatusCode::BadViewVersionInvalid => true,
            StatusCode::UncertainNotAllNodesAvailable => true,
            StatusCode::GoodResultsMayBeIncomplete => true,
            StatusCode::BadNotTypeDefinition => true,
            StatusCode::UncertainReferenceOutOfServer => true,
            StatusCode::BadTooManyMatches => true,
            StatusCode::BadQueryTooComplex => true,
            StatusCode::BadNoMatch => true,
            StatusCode::BadMaxAgeInvalid => true,
            StatusCode::BadSecurityModeInsufficient => true,
            StatusCode::BadHistoryOperationInvalid => true,
            StatusCode::BadHistoryOperationUnsupported => true,
            StatusCode::BadInvalidTimestampArgument => true,
            StatusCode::BadWriteNotSupported => true,
            StatusCode::BadTypeMismatch => true,
            StatusCode::BadMethodInvalid => true,
            StatusCode::BadArgumentsMissing => true,
            StatusCode::BadNotExecutable => true,
            StatusCode::BadTooManySubscriptions => true,
            StatusCode::BadTooManyPublishRequests => true,
            StatusCode::BadNoSubscription => true,
            StatusCode::BadSequenceNumberUnknown => true,
            StatusCode::GoodRetransmissionQueueNotSupported => true,
            StatusCode::BadMessageNotAvailable => true,
            StatusCode::BadInsufficientClientProfile => true,
            StatusCode::BadStateNotActive => true,
            StatusCode::BadAlreadyExists => true,
            StatusCode::BadTcpServerTooBusy => true,
            StatusCode::BadTcpMessageTypeInvalid => true,
            StatusCode::BadTcpSecureChannelUnknown => true,
            StatusCode::BadTcpMessageTooLarge => true,
            StatusCode::BadTcpNotEnoughResources => true,
            StatusCode::BadTcpInternalError => true,
            StatusCode::BadTcpEndpointUrlInvalid => true,
            StatusCode::BadRequestInterrupted => true,
            StatusCode::BadRequestTimeout => true,
            StatusCode::BadSecureChannelClosed => true,
            StatusCode::BadSecureChannelTokenUnknown => true,
            StatusCode::BadSequenceNumberInvalid => true,
            StatusCode::BadProtocolVersionUnsupported => true,
            StatusCode::BadConfigurationError => true,
            StatusCode::BadNotConnected => true,
            StatusCode::BadDeviceFailure => true,
            StatusCode::BadSensorFailure => true,
            StatusCode::BadOutOfService => true,
            StatusCode::BadDeadbandFilterInvalid => true,
            StatusCode::UncertainNoCommunicationLastUsableValue => true,
            StatusCode::UncertainLastUsableValue => true,
            StatusCode::UncertainSubstituteValue => true,
            StatusCode::UncertainInitialValue => true,
            StatusCode::UncertainSensorNotAccurate => true,
            StatusCode::UncertainEngineeringUnitsExceeded => true,
            StatusCode::UncertainSubNormal => true,
            StatusCode::GoodLocalOverride => true,
            StatusCode::BadRefreshInProgress => true,
            StatusCode::BadConditionAlreadyDisabled => true,
            StatusCode::BadConditionAlreadyEnabled => true,
            StatusCode::BadConditionDisabled => true,
            StatusCode::BadEventIdUnknown => true,
            StatusCode::BadEventNotAcknowledgeable => true,
            StatusCode::BadDialogNotActive => true,
            StatusCode::BadDialogResponseInvalid => true,
            StatusCode::BadConditionBranchAlreadyAcked => true,
            StatusCode::BadConditionBranchAlreadyConfirmed => true,
            StatusCode::BadConditionAlreadyShelved => true,
            StatusCode::BadConditionNotShelved => true,
            StatusCode::BadShelvingTimeOutOfRange => true,
            StatusCode::BadNoData => true,
            StatusCode::BadBoundNotFound => true,
            StatusCode::BadBoundNotSupported => true,
            StatusCode::BadDataLost => true,
            StatusCode::BadDataUnavailable => true,
            StatusCode::BadEntryExists => true,
            StatusCode::BadNoEntryExists => true,
            StatusCode::BadTimestampNotSupported => true,
            StatusCode::GoodEntryInserted => true,
            StatusCode::GoodEntryReplaced => true,
            StatusCode::UncertainDataSubNormal => true,
            StatusCode::GoodNoData => true,
            StatusCode::GoodMoreData => true,
            StatusCode::BadAggregateListMismatch => true,
            StatusCode::BadAggregateNotSupported => true,
            StatusCode::BadAggregateInvalidInputs => true,
            StatusCode::BadAggregateConfigurationRejected => true,
            StatusCode::GoodDataIgnored => true,
            StatusCode::BadRequestNotAllowed => true,
            StatusCode::BadRequestNotComplete => true,
            StatusCode::BadTicketRequired => true,
            StatusCode::BadTicketInvalid => true,
            StatusCode::GoodEdited => true,
            StatusCode::GoodPostActionFailed => true,
            StatusCode::UncertainDominantValueChanged => true,
            StatusCode::GoodDependentValueChanged => true,
            StatusCode::BadDominantValueChanged => true,
            StatusCode::UncertainDependentValueChanged => true,
            StatusCode::BadDependentValueChanged => true,
            StatusCode::GoodEdited_DependentValueChanged => true,
            StatusCode::GoodEdited_DominantValueChanged => true,
            StatusCode::GoodEdited_DominantValueChanged_DependentValueChanged => true,
            StatusCode::BadEdited_OutOfRange => true,
            StatusCode::BadInitialValue_OutOfRange => true,
            StatusCode::BadOutOfRange_DominantValueChanged => true,
            StatusCode::BadEdited_OutOfRange_DominantValueChanged => true,
            StatusCode::BadOutOfRange_DominantValueChanged_DependentValueChanged => true,
            StatusCode::BadEdited_OutOfRange_DominantValueChanged_DependentValueChanged => true,
            StatusCode::GoodCommunicationEvent => true,
            StatusCode::GoodShutdownEvent => true,
            StatusCode::GoodCallAgain => true,
            StatusCode::GoodNonCriticalTimeout => true,
            StatusCode::BadInvalidArgument => true,
            StatusCode::BadConnectionRejected => true,
            StatusCode::BadDisconnect => true,
            StatusCode::BadConnectionClosed => true,
            StatusCode::BadInvalidState => true,
            StatusCode::BadEndOfStream => true,
            StatusCode::BadNoDataAvailable => true,
            StatusCode::BadWaitingForResponse => true,
            StatusCode::BadOperationAbandoned => true,
            StatusCode::BadExpectedStreamToBlock => true,
            StatusCode::BadWouldBlock => true,
            StatusCode::BadSyntaxError => true,
            StatusCode::BadMaxConnectionsReached => true,
            StatusCode::UncertainTransducerInManual => true,
            StatusCode::UncertainSimulatedValue => true,
            StatusCode::UncertainSensorCalibration => true,
            StatusCode::UncertainConfigurationError => true,
            StatusCode::GoodCascadeInitializationAcknowledged => true,
            StatusCode::GoodCascadeInitializationRequest => true,
            StatusCode::GoodCascadeNotInvited => true,
            StatusCode::GoodCascadeNotSelected => true,
            StatusCode::GoodFaultStateActive => true,
            StatusCode::GoodInitiateFaultState => true,
            StatusCode::GoodCascade => true,
            StatusCode::BadDataSetIdInvalid => true,
            _ => false,
        }
    }
    pub(crate) fn get_error_name(error:u32){
        if !StatusCode::is_status_code(&error){
            println!("not a status code");
        }
        else{
            match error{
                0=>{println!("Good");}
                1073741824=>{println!("Uncertain");}
                2147483648=>{println!("Bad");}
                2147549184=>{println!("BadUnexpectedError");}
                2147614720=>{println!("BadInternalError");}
                2147680256=>{println!("BadOutOfMemory");}
                2147745792=>{println!("BadResourceUnavailable");}
                2147811328=>{println!("BadCommunicationError");}
                2147876864=>{println!("BadEncodingError");}
                2147942400=>{println!("BadDecodingError");}
                2148007936=>{println!("BadEncodingLimitsExceeded");}
                2159542272=>{println!("BadRequestTooLarge");}
                2159607808=>{println!("BadResponseTooLarge");}
                2148073472=>{println!("BadUnknownResponse");}
                2148139008=>{println!("BadTimeout");}
                2148204544=>{println!("BadServiceUnsupported");}
                2148270080=>{println!("BadShutdown");}
                2148335616=>{println!("BadServerNotConnected");}
                2148401152=>{println!("BadServerHalted");}
                2148466688=>{println!("BadNothingToDo");}
                2148532224=>{println!("BadTooManyOperations");}
                2161836032=>{println!("BadTooManyMonitoredItems");}
                2148597760=>{println!("BadDataTypeIdUnknown");}
                2148663296=>{println!("BadCertificateInvalid");}
                2148728832=>{println!("BadSecurityChecksFailed");}
                2165571584=>{println!("BadCertificatePolicyCheckFailed");}
                2148794368=>{println!("BadCertificateTimeInvalid");}
                2148859904=>{println!("BadCertificateIssuerTimeInvalid");}
                2148925440=>{println!("BadCertificateHostNameInvalid");}
                2148990976=>{println!("BadCertificateUriInvalid");}
                2149056512=>{println!("BadCertificateUseNotAllowed");}
                2149122048=>{println!("BadCertificateIssuerUseNotAllowed");}
                2149187584=>{println!("BadCertificateUntrusted");}
                2149253120=>{println!("BadCertificateRevocationUnknown");}
                2149318656=>{println!("BadCertificateIssuerRevocationUnknown");}
                2149384192=>{println!("BadCertificateRevoked");}
                2149449728=>{println!("BadCertificateIssuerRevoked");}
                2165112832=>{println!("BadCertificateChainIncomplete");}
                2149515264=>{println!("BadUserAccessDenied");}
                2149580800=>{println!("BadIdentityTokenInvalid");}
                2149646336=>{println!("BadIdentityTokenRejected");}
                2149711872=>{println!("BadSecureChannelIdInvalid");}
                2149777408=>{println!("BadInvalidTimestamp");}
                2149842944=>{println!("BadNonceInvalid");}
                2149908480=>{println!("BadSessionIdInvalid");}
                2149974016=>{println!("BadSessionClosed");}
                2150039552=>{println!("BadSessionNotActivated");}
                2150105088=>{println!("BadSubscriptionIdInvalid");}
                2150236160=>{println!("BadRequestHeaderInvalid");}
                2150301696=>{println!("BadTimestampsToReturnInvalid");}
                2150367232=>{println!("BadRequestCancelledByClient");}
                2162491392=>{println!("BadTooManyArguments");}
                2165178368=>{println!("BadLicenseExpired");}
                2165243904=>{println!("BadLicenseLimitsExceeded");}
                2165309440=>{println!("BadLicenseNotAvailable");}
                2949120=>{println!("GoodSubscriptionTransferred");}
                3014656=>{println!("GoodCompletesAsynchronously");}
                3080192=>{println!("GoodOverload");}
                3145728=>{println!("GoodClamped");}
                2150694912=>{println!("BadNoCommunication");}
                2150760448=>{println!("BadWaitingForInitialData");}
                2150825984=>{println!("BadNodeIdInvalid");}
                2150891520=>{println!("BadNodeIdUnknown");}
                2150957056=>{println!("BadAttributeIdInvalid");}
                2151022592=>{println!("BadIndexRangeInvalid");}
                2151088128=>{println!("BadIndexRangeNoData");}
                2151153664=>{println!("BadDataEncodingInvalid");}
                2151219200=>{println!("BadDataEncodingUnsupported");}
                2151284736=>{println!("BadNotReadable");}
                2151350272=>{println!("BadNotWritable");}
                2151415808=>{println!("BadOutOfRange");}
                2151481344=>{println!("BadNotSupported");}
                2151546880=>{println!("BadNotFound");}
                2151612416=>{println!("BadObjectDeleted");}
                2151677952=>{println!("BadNotImplemented");}
                2151743488=>{println!("BadMonitoringModeInvalid");}
                2151809024=>{println!("BadMonitoredItemIdInvalid");}
                2151874560=>{println!("BadMonitoredItemFilterInvalid");}
                2151940096=>{println!("BadMonitoredItemFilterUnsupported");}
                2152005632=>{println!("BadFilterNotAllowed");}
                2152071168=>{println!("BadStructureMissing");}
                2152136704=>{println!("BadEventFilterInvalid");}
                2152202240=>{println!("BadContentFilterInvalid");}
                2160132096=>{println!("BadFilterOperatorInvalid");}
                2160197632=>{println!("BadFilterOperatorUnsupported");}
                2160263168=>{println!("BadFilterOperandCountMismatch");}
                2152267776=>{println!("BadFilterOperandInvalid");}
                2160328704=>{println!("BadFilterElementInvalid");}
                2160394240=>{println!("BadFilterLiteralInvalid");}
                2152333312=>{println!("BadContinuationPointInvalid");}
                2152398848=>{println!("BadNoContinuationPoints");}
                2152464384=>{println!("BadReferenceTypeIdInvalid");}
                2152529920=>{println!("BadBrowseDirectionInvalid");}
                2152595456=>{println!("BadNodeNotInView");}
                2165440512=>{println!("BadNumericOverflow");}
                2152660992=>{println!("BadServerUriInvalid");}
                2152726528=>{println!("BadServerNameMissing");}
                2152792064=>{println!("BadDiscoveryUrlMissing");}
                2152857600=>{println!("BadSempahoreFileMissing");}
                2152923136=>{println!("BadRequestTypeInvalid");}
                2152988672=>{println!("BadSecurityModeRejected");}
                2153054208=>{println!("BadSecurityPolicyRejected");}
                2153119744=>{println!("BadTooManySessions");}
                2153185280=>{println!("BadUserSignatureInvalid");}
                2153250816=>{println!("BadApplicationSignatureInvalid");}
                2153316352=>{println!("BadNoValidCertificates");}
                2160459776=>{println!("BadIdentityChangeNotSupported");}
                2153381888=>{println!("BadRequestCancelledByRequest");}
                2153447424=>{println!("BadParentNodeIdInvalid");}
                2153512960=>{println!("BadReferenceNotAllowed");}
                2153578496=>{println!("BadNodeIdRejected");}
                2153644032=>{println!("BadNodeIdExists");}
                2153709568=>{println!("BadNodeClassInvalid");}
                2153775104=>{println!("BadBrowseNameInvalid");}
                2153840640=>{println!("BadBrowseNameDuplicated");}
                2153906176=>{println!("BadNodeAttributesInvalid");}
                2153971712=>{println!("BadTypeDefinitionInvalid");}
                2154037248=>{println!("BadSourceNodeIdInvalid");}
                2154102784=>{println!("BadTargetNodeIdInvalid");}
                2154168320=>{println!("BadDuplicateReferenceNotAllowed");}
                2154233856=>{println!("BadInvalidSelfReference");}
                2154299392=>{println!("BadReferenceLocalOnly");}
                2154364928=>{println!("BadNoDeleteRights");}
                1086062592=>{println!("UncertainReferenceNotDeleted");}
                2154430464=>{println!("BadServerIndexInvalid");}
                2154496000=>{println!("BadViewIdUnknown");}
                2160656384=>{println!("BadViewTimestampInvalid");}
                2160721920=>{println!("BadViewParameterMismatch");}
                2160787456=>{println!("BadViewVersionInvalid");}
                1086324736=>{println!("UncertainNotAllNodesAvailable");}
                12189696=>{println!("GoodResultsMayBeIncomplete");}
                2160590848=>{println!("BadNotTypeDefinition");}
                1080819712=>{println!("UncertainReferenceOutOfServer");}
                2154627072=>{println!("BadTooManyMatches");}
                2154692608=>{println!("BadQueryTooComplex");}
                2154758144=>{println!("BadNoMatch");}
                2154823680=>{println!("BadMaxAgeInvalid");}
                2162556928=>{println!("BadSecurityModeInsufficient");}
                2154889216=>{println!("BadHistoryOperationInvalid");}
                2154954752=>{println!("BadHistoryOperationUnsupported");}
                2159869952=>{println!("BadInvalidTimestampArgument");}
                2155020288=>{println!("BadWriteNotSupported");}
                2155085824=>{println!("BadTypeMismatch");}
                2155151360=>{println!("BadMethodInvalid");}
                2155216896=>{println!("BadArgumentsMissing");}
                2165374976=>{println!("BadNotExecutable");}
                2155282432=>{println!("BadTooManySubscriptions");}
                2155347968=>{println!("BadTooManyPublishRequests");}
                2155413504=>{println!("BadNoSubscription");}
                2155479040=>{println!("BadSequenceNumberUnknown");}
                14614528=>{println!("GoodRetransmissionQueueNotSupported");}
                2155544576=>{println!("BadMessageNotAvailable");}
                2155610112=>{println!("BadInsufficientClientProfile");}
                2160001024=>{println!("BadStateNotActive");}
                2165637120=>{println!("BadAlreadyExists");}
                2155675648=>{println!("BadTcpServerTooBusy");}
                2155741184=>{println!("BadTcpMessageTypeInvalid");}
                2155806720=>{println!("BadTcpSecureChannelUnknown");}
                2155872256=>{println!("BadTcpMessageTooLarge");}
                2155937792=>{println!("BadTcpNotEnoughResources");}
                2156003328=>{println!("BadTcpInternalError");}
                2156068864=>{println!("BadTcpEndpointUrlInvalid");}
                2156134400=>{println!("BadRequestInterrupted");}
                2156199936=>{println!("BadRequestTimeout");}
                2156265472=>{println!("BadSecureChannelClosed");}
                2156331008=>{println!("BadSecureChannelTokenUnknown");}
                2156396544=>{println!("BadSequenceNumberInvalid");}
                2159935488=>{println!("BadProtocolVersionUnsupported");}
                2156462080=>{println!("BadConfigurationError");}
                2156527616=>{println!("BadNotConnected");}
                2156593152=>{println!("BadDeviceFailure");}
                2156658688=>{println!("BadSensorFailure");}
                2156724224=>{println!("BadOutOfService");}
                2156789760=>{println!("BadDeadbandFilterInvalid");}
                1083113472=>{println!("UncertainNoCommunicationLastUsableValue");}
                1083179008=>{println!("UncertainLastUsableValue");}
                1083244544=>{println!("UncertainSubstituteValue");}
                1083310080=>{println!("UncertainInitialValue");}
                1083375616=>{println!("UncertainSensorNotAccurate");}
                1083441152=>{println!("UncertainEngineeringUnitsExceeded");}
                1083506688=>{println!("UncertainSubNormal");}
                9830400=>{println!("GoodLocalOverride");}
                2157379584=>{println!("BadRefreshInProgress");}
                2157445120=>{println!("BadConditionAlreadyDisabled");}
                2160852992=>{println!("BadConditionAlreadyEnabled");}
                2157510656=>{println!("BadConditionDisabled");}
                2157576192=>{println!("BadEventIdUnknown");}
                2159738880=>{println!("BadEventNotAcknowledgeable");}
                2160918528=>{println!("BadDialogNotActive");}
                2160984064=>{println!("BadDialogResponseInvalid");}
                2161049600=>{println!("BadConditionBranchAlreadyAcked");}
                2161115136=>{println!("BadConditionBranchAlreadyConfirmed");}
                2161180672=>{println!("BadConditionAlreadyShelved");}
                2161246208=>{println!("BadConditionNotShelved");}
                2161311744=>{println!("BadShelvingTimeOutOfRange");}
                2157641728=>{println!("BadNoData");}
                2161573888=>{println!("BadBoundNotFound");}
                2161639424=>{println!("BadBoundNotSupported");}
                2157772800=>{println!("BadDataLost");}
                2157838336=>{println!("BadDataUnavailable");}
                2157903872=>{println!("BadEntryExists");}
                2157969408=>{println!("BadNoEntryExists");}
                2158034944=>{println!("BadTimestampNotSupported");}
                10616832=>{println!("GoodEntryInserted");}
                10682368=>{println!("GoodEntryReplaced");}
                1084489728=>{println!("UncertainDataSubNormal");}
                10813440=>{println!("GoodNoData");}
                10878976=>{println!("GoodMoreData");}
                2161377280=>{println!("BadAggregateListMismatch");}
                2161442816=>{println!("BadAggregateNotSupported");}
                2161508352=>{println!("BadAggregateInvalidInputs");}
                2161770496=>{println!("BadAggregateConfigurationRejected");}
                14221312=>{println!("GoodDataIgnored");}
                2162425856=>{println!("BadRequestNotAllowed");}
                2165506048=>{println!("BadRequestNotComplete");}
                2166292480=>{println!("BadTicketRequired");}
                2166358016=>{println!("BadTicketInvalid");}
                14417920=>{println!("GoodEdited");}
                14483456=>{println!("GoodPostActionFailed");}
                1088290816=>{println!("UncertainDominantValueChanged");}
                14680064=>{println!("GoodDependentValueChanged");}
                2162229248=>{println!("BadDominantValueChanged");}
                1088552960=>{println!("UncertainDependentValueChanged");}
                2162360320=>{println!("BadDependentValueChanged");}
                18219008=>{println!("GoodEdited_DependentValueChanged");}
                18284544=>{println!("GoodEdited_DominantValueChanged");}
                18350080=>{println!("GoodEdited_DominantValueChanged_DependentValueChanged");}
                2165899264=>{println!("BadEdited_OutOfRange");}
                2165964800=>{println!("BadInitialValue_OutOfRange");}
                2166030336=>{println!("BadOutOfRange_DominantValueChanged");}
                2166095872=>{println!("BadEdited_OutOfRange_DominantValueChanged");}
                2166161408=>{println!("BadOutOfRange_DominantValueChanged_DependentValueChanged");}
                2166226944=>{println!("BadEdited_OutOfRange_DominantValueChanged_DependentValueChanged");}
                10944512=>{println!("GoodCommunicationEvent");}
                11010048=>{println!("GoodShutdownEvent");}
                11075584=>{println!("GoodCallAgain");}
                11141120=>{println!("GoodNonCriticalTimeout");}
                2158690304=>{println!("BadInvalidArgument");}
                2158755840=>{println!("BadConnectionRejected");}
                2158821376=>{println!("BadDisconnect");}
                2158886912=>{println!("BadConnectionClosed");}
                2158952448=>{println!("BadInvalidState");}
                2159017984=>{println!("BadEndOfStream");}
                2159083520=>{println!("BadNoDataAvailable");}
                2159149056=>{println!("BadWaitingForResponse");}
                2159214592=>{println!("BadOperationAbandoned");}
                2159280128=>{println!("BadExpectedStreamToBlock");}
                2159345664=>{println!("BadWouldBlock");}
                2159411200=>{println!("BadSyntaxError");}
                2159476736=>{println!("BadMaxConnectionsReached");}
                1107820544=>{println!("UncertainTransducerInManual");}
                1107886080=>{println!("UncertainSimulatedValue");}
                1107951616=>{println!("UncertainSensorCalibration");}
                1108279296=>{println!("UncertainConfigurationError");}
                67174400=>{println!("GoodCascadeInitializationAcknowledged");}
                67239936=>{println!("GoodCascadeInitializationRequest");}
                67305472=>{println!("GoodCascadeNotInvited");}
                67371008=>{println!("GoodCascadeNotSelected");}
                67567616=>{println!("GoodFaultStateActive");}
                67633152=>{println!("GoodInitiateFaultState");}
                67698688=>{println!("GoodCascade");}
                2162622464=>{println!("BadDataSetIdInvalid");}
                _=>{println!("not a status code");}
            }
        }
    }

    pub(crate) fn to_str(&self)->&'static str{
        if !StatusCode::is_status_code(&self.0){
            return "not a status code"
        }
        else{
            match &self.0{
                0=>{"Good"}
                1073741824=>{"Uncertain"}
                2147483648=>{"Bad"}
                2147549184=>{"BadUnexpectedError"}
                2147614720=>{"BadInternalError"}
                2147680256=>{"BadOutOfMemory"}
                2147745792=>{"BadResourceUnavailable"}
                2147811328=>{"BadCommunicationError"}
                2147876864=>{"BadEncodingError"}
                2147942400=>{"BadDecodingError"}
                2148007936=>{"BadEncodingLimitsExceeded"}
                2159542272=>{"BadRequestTooLarge"}
                2159607808=>{"BadResponseTooLarge"}
                2148073472=>{"BadUnknownResponse"}
                2148139008=>{"BadTimeout"}
                2148204544=>{"BadServiceUnsupported"}
                2148270080=>{"BadShutdown"}
                2148335616=>{"BadServerNotConnected"}
                2148401152=>{"BadServerHalted"}
                2148466688=>{"BadNothingToDo"}
                2148532224=>{"BadTooManyOperations"}
                2161836032=>{"BadTooManyMonitoredItems"}
                2148597760=>{"BadDataTypeIdUnknown"}
                2148663296=>{"BadCertificateInvalid"}
                2148728832=>{"BadSecurityChecksFailed"}
                2165571584=>{"BadCertificatePolicyCheckFailed"}
                2148794368=>{"BadCertificateTimeInvalid"}
                2148859904=>{"BadCertificateIssuerTimeInvalid"}
                2148925440=>{"BadCertificateHostNameInvalid"}
                2148990976=>{"BadCertificateUriInvalid"}
                2149056512=>{"BadCertificateUseNotAllowed"}
                2149122048=>{"BadCertificateIssuerUseNotAllowed"}
                2149187584=>{"BadCertificateUntrusted"}
                2149253120=>{"BadCertificateRevocationUnknown"}
                2149318656=>{"BadCertificateIssuerRevocationUnknown"}
                2149384192=>{"BadCertificateRevoked"}
                2149449728=>{"BadCertificateIssuerRevoked"}
                2165112832=>{"BadCertificateChainIncomplete"}
                2149515264=>{"BadUserAccessDenied"}
                2149580800=>{"BadIdentityTokenInvalid"}
                2149646336=>{"BadIdentityTokenRejected"}
                2149711872=>{"BadSecureChannelIdInvalid"}
                2149777408=>{"BadInvalidTimestamp"}
                2149842944=>{"BadNonceInvalid"}
                2149908480=>{"BadSessionIdInvalid"}
                2149974016=>{"BadSessionClosed"}
                2150039552=>{"BadSessionNotActivated"}
                2150105088=>{"BadSubscriptionIdInvalid"}
                2150236160=>{"BadRequestHeaderInvalid"}
                2150301696=>{"BadTimestampsToReturnInvalid"}
                2150367232=>{"BadRequestCancelledByClient"}
                2162491392=>{"BadTooManyArguments"}
                2165178368=>{"BadLicenseExpired"}
                2165243904=>{"BadLicenseLimitsExceeded"}
                2165309440=>{"BadLicenseNotAvailable"}
                2949120=>{"GoodSubscriptionTransferred"}
                3014656=>{"GoodCompletesAsynchronously"}
                3080192=>{"GoodOverload"}
                3145728=>{"GoodClamped"}
                2150694912=>{"BadNoCommunication"}
                2150760448=>{"BadWaitingForInitialData"}
                2150825984=>{"BadNodeIdInvalid"}
                2150891520=>{"BadNodeIdUnknown"}
                2150957056=>{"BadAttributeIdInvalid"}
                2151022592=>{"BadIndexRangeInvalid"}
                2151088128=>{"BadIndexRangeNoData"}
                2151153664=>{"BadDataEncodingInvalid"}
                2151219200=>{"BadDataEncodingUnsupported"}
                2151284736=>{"BadNotReadable"}
                2151350272=>{"BadNotWritable"}
                2151415808=>{"BadOutOfRange"}
                2151481344=>{"BadNotSupported"}
                2151546880=>{"BadNotFound"}
                2151612416=>{"BadObjectDeleted"}
                2151677952=>{"BadNotImplemented"}
                2151743488=>{"BadMonitoringModeInvalid"}
                2151809024=>{"BadMonitoredItemIdInvalid"}
                2151874560=>{"BadMonitoredItemFilterInvalid"}
                2151940096=>{"BadMonitoredItemFilterUnsupported"}
                2152005632=>{"BadFilterNotAllowed"}
                2152071168=>{"BadStructureMissing"}
                2152136704=>{"BadEventFilterInvalid"}
                2152202240=>{"BadContentFilterInvalid"}
                2160132096=>{"BadFilterOperatorInvalid"}
                2160197632=>{"BadFilterOperatorUnsupported"}
                2160263168=>{"BadFilterOperandCountMismatch"}
                2152267776=>{"BadFilterOperandInvalid"}
                2160328704=>{"BadFilterElementInvalid"}
                2160394240=>{"BadFilterLiteralInvalid"}
                2152333312=>{"BadContinuationPointInvalid"}
                2152398848=>{"BadNoContinuationPoints"}
                2152464384=>{"BadReferenceTypeIdInvalid"}
                2152529920=>{"BadBrowseDirectionInvalid"}
                2152595456=>{"BadNodeNotInView"}
                2165440512=>{"BadNumericOverflow"}
                2152660992=>{"BadServerUriInvalid"}
                2152726528=>{"BadServerNameMissing"}
                2152792064=>{"BadDiscoveryUrlMissing"}
                2152857600=>{"BadSempahoreFileMissing"}
                2152923136=>{"BadRequestTypeInvalid"}
                2152988672=>{"BadSecurityModeRejected"}
                2153054208=>{"BadSecurityPolicyRejected"}
                2153119744=>{"BadTooManySessions"}
                2153185280=>{"BadUserSignatureInvalid"}
                2153250816=>{"BadApplicationSignatureInvalid"}
                2153316352=>{"BadNoValidCertificates"}
                2160459776=>{"BadIdentityChangeNotSupported"}
                2153381888=>{"BadRequestCancelledByRequest"}
                2153447424=>{"BadParentNodeIdInvalid"}
                2153512960=>{"BadReferenceNotAllowed"}
                2153578496=>{"BadNodeIdRejected"}
                2153644032=>{"BadNodeIdExists"}
                2153709568=>{"BadNodeClassInvalid"}
                2153775104=>{"BadBrowseNameInvalid"}
                2153840640=>{"BadBrowseNameDuplicated"}
                2153906176=>{"BadNodeAttributesInvalid"}
                2153971712=>{"BadTypeDefinitionInvalid"}
                2154037248=>{"BadSourceNodeIdInvalid"}
                2154102784=>{"BadTargetNodeIdInvalid"}
                2154168320=>{"BadDuplicateReferenceNotAllowed"}
                2154233856=>{"BadInvalidSelfReference"}
                2154299392=>{"BadReferenceLocalOnly"}
                2154364928=>{"BadNoDeleteRights"}
                1086062592=>{"UncertainReferenceNotDeleted"}
                2154430464=>{"BadServerIndexInvalid"}
                2154496000=>{"BadViewIdUnknown"}
                2160656384=>{"BadViewTimestampInvalid"}
                2160721920=>{"BadViewParameterMismatch"}
                2160787456=>{"BadViewVersionInvalid"}
                1086324736=>{"UncertainNotAllNodesAvailable"}
                12189696=>{"GoodResultsMayBeIncomplete"}
                2160590848=>{"BadNotTypeDefinition"}
                1080819712=>{"UncertainReferenceOutOfServer"}
                2154627072=>{"BadTooManyMatches"}
                2154692608=>{"BadQueryTooComplex"}
                2154758144=>{"BadNoMatch"}
                2154823680=>{"BadMaxAgeInvalid"}
                2162556928=>{"BadSecurityModeInsufficient"}
                2154889216=>{"BadHistoryOperationInvalid"}
                2154954752=>{"BadHistoryOperationUnsupported"}
                2159869952=>{"BadInvalidTimestampArgument"}
                2155020288=>{"BadWriteNotSupported"}
                2155085824=>{"BadTypeMismatch"}
                2155151360=>{"BadMethodInvalid"}
                2155216896=>{"BadArgumentsMissing"}
                2165374976=>{"BadNotExecutable"}
                2155282432=>{"BadTooManySubscriptions"}
                2155347968=>{"BadTooManyPublishRequests"}
                2155413504=>{"BadNoSubscription"}
                2155479040=>{"BadSequenceNumberUnknown"}
                14614528=>{"GoodRetransmissionQueueNotSupported"}
                2155544576=>{"BadMessageNotAvailable"}
                2155610112=>{"BadInsufficientClientProfile"}
                2160001024=>{"BadStateNotActive"}
                2165637120=>{"BadAlreadyExists"}
                2155675648=>{"BadTcpServerTooBusy"}
                2155741184=>{"BadTcpMessageTypeInvalid"}
                2155806720=>{"BadTcpSecureChannelUnknown"}
                2155872256=>{"BadTcpMessageTooLarge"}
                2155937792=>{"BadTcpNotEnoughResources"}
                2156003328=>{"BadTcpInternalError"}
                2156068864=>{"BadTcpEndpointUrlInvalid"}
                2156134400=>{"BadRequestInterrupted"}
                2156199936=>{"BadRequestTimeout"}
                2156265472=>{"BadSecureChannelClosed"}
                2156331008=>{"BadSecureChannelTokenUnknown"}
                2156396544=>{"BadSequenceNumberInvalid"}
                2159935488=>{"BadProtocolVersionUnsupported"}
                2156462080=>{"BadConfigurationError"}
                2156527616=>{"BadNotConnected"}
                2156593152=>{"BadDeviceFailure"}
                2156658688=>{"BadSensorFailure"}
                2156724224=>{"BadOutOfService"}
                2156789760=>{"BadDeadbandFilterInvalid"}
                1083113472=>{"UncertainNoCommunicationLastUsableValue"}
                1083179008=>{"UncertainLastUsableValue"}
                1083244544=>{"UncertainSubstituteValue"}
                1083310080=>{"UncertainInitialValue"}
                1083375616=>{"UncertainSensorNotAccurate"}
                1083441152=>{"UncertainEngineeringUnitsExceeded"}
                1083506688=>{"UncertainSubNormal"}
                9830400=>{"GoodLocalOverride"}
                2157379584=>{"BadRefreshInProgress"}
                2157445120=>{"BadConditionAlreadyDisabled"}
                2160852992=>{"BadConditionAlreadyEnabled"}
                2157510656=>{"BadConditionDisabled"}
                2157576192=>{"BadEventIdUnknown"}
                2159738880=>{"BadEventNotAcknowledgeable"}
                2160918528=>{"BadDialogNotActive"}
                2160984064=>{"BadDialogResponseInvalid"}
                2161049600=>{"BadConditionBranchAlreadyAcked"}
                2161115136=>{"BadConditionBranchAlreadyConfirmed"}
                2161180672=>{"BadConditionAlreadyShelved"}
                2161246208=>{"BadConditionNotShelved"}
                2161311744=>{"BadShelvingTimeOutOfRange"}
                2157641728=>{"BadNoData"}
                2161573888=>{"BadBoundNotFound"}
                2161639424=>{"BadBoundNotSupported"}
                2157772800=>{"BadDataLost"}
                2157838336=>{"BadDataUnavailable"}
                2157903872=>{"BadEntryExists"}
                2157969408=>{"BadNoEntryExists"}
                2158034944=>{"BadTimestampNotSupported"}
                10616832=>{"GoodEntryInserted"}
                10682368=>{"GoodEntryReplaced"}
                1084489728=>{"UncertainDataSubNormal"}
                10813440=>{"GoodNoData"}
                10878976=>{"GoodMoreData"}
                2161377280=>{"BadAggregateListMismatch"}
                2161442816=>{"BadAggregateNotSupported"}
                2161508352=>{"BadAggregateInvalidInputs"}
                2161770496=>{"BadAggregateConfigurationRejected"}
                14221312=>{"GoodDataIgnored"}
                2162425856=>{"BadRequestNotAllowed"}
                2165506048=>{"BadRequestNotComplete"}
                2166292480=>{"BadTicketRequired"}
                2166358016=>{"BadTicketInvalid"}
                14417920=>{"GoodEdited"}
                14483456=>{"GoodPostActionFailed"}
                1088290816=>{"UncertainDominantValueChanged"}
                14680064=>{"GoodDependentValueChanged"}
                2162229248=>{"BadDominantValueChanged"}
                1088552960=>{"UncertainDependentValueChanged"}
                2162360320=>{"BadDependentValueChanged"}
                18219008=>{"GoodEdited_DependentValueChanged"}
                18284544=>{"GoodEdited_DominantValueChanged"}
                18350080=>{"GoodEdited_DominantValueChanged_DependentValueChanged"}
                2165899264=>{"BadEdited_OutOfRange"}
                2165964800=>{"BadInitialValue_OutOfRange"}
                2166030336=>{"BadOutOfRange_DominantValueChanged"}
                2166095872=>{"BadEdited_OutOfRange_DominantValueChanged"}
                2166161408=>{"BadOutOfRange_DominantValueChanged_DependentValueChanged"}
                2166226944=>{"BadEdited_OutOfRange_DominantValueChanged_DependentValueChanged"}
                10944512=>{"GoodCommunicationEvent"}
                11010048=>{"GoodShutdownEvent"}
                11075584=>{"GoodCallAgain"}
                11141120=>{"GoodNonCriticalTimeout"}
                2158690304=>{"BadInvalidArgument"}
                2158755840=>{"BadConnectionRejected"}
                2158821376=>{"BadDisconnect"}
                2158886912=>{"BadConnectionClosed"}
                2158952448=>{"BadInvalidState"}
                2159017984=>{"BadEndOfStream"}
                2159083520=>{"BadNoDataAvailable"}
                2159149056=>{"BadWaitingForResponse"}
                2159214592=>{"BadOperationAbandoned"}
                2159280128=>{"BadExpectedStreamToBlock"}
                2159345664=>{"BadWouldBlock"}
                2159411200=>{"BadSyntaxError"}
                2159476736=>{"BadMaxConnectionsReached"}
                1107820544=>{"UncertainTransducerInManual"}
                1107886080=>{"UncertainSimulatedValue"}
                1107951616=>{"UncertainSensorCalibration"}
                1108279296=>{"UncertainConfigurationError"}
                67174400=>{"GoodCascadeInitializationAcknowledged"}
                67239936=>{"GoodCascadeInitializationRequest"}
                67305472=>{"GoodCascadeNotInvited"}
                67371008=>{"GoodCascadeNotSelected"}
                67567616=>{"GoodFaultStateActive"}
                67633152=>{"GoodInitiateFaultState"}
                67698688=>{"GoodCascade"}
                2162622464=>{"BadDataSetIdInvalid"}
                _=>{"not a status code"}
            }
        }
    }
}

impl Serialize for StatusCode {
    fn serialize(&self) -> Vec<u8> {
        self.0.serialize()
    }
}

impl Deserialize for StatusCode {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, status_code) = u32::deserialize(data)?;
        if StatusCode::is_status_code(&status_code) {
            Ok((data, StatusCode(status_code)))
        } else {
            Ok((data, StatusCode(StatusCode::Uncertain)))
        }
    }
}
