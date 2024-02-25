use crate::crypto::security_policy::SecurityPolicy;
use crate::encoding_prelude::*;

use super::header::prelude::*;
use crate::uatypes::prelude::*;
use crate::{
    crypto::security_policy::SecurityPolicyUri
};

#[derive(Debug,Serialize,Deserialize)]
pub struct OpenSecureChannelRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: AsymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) client_protocol_version: u32,
    pub(crate) request_type: u32,  //SecurityTokenRequestType,
    pub(crate) security_mode: u32, //MessageSecurityMode,
    pub(crate) client_nonce: ByteString,
    pub(crate) requested_lifetime: u32,
}

impl Default for OpenSecureChannelRequest {
    fn default() -> Self {
        let message_header: MessageHeader = Default::default();
        let security_header: AsymmetricSecurityHeader = Default::default();
        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 1,
            request_id: 1,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 446);
        let request_header = RequestHeader {
            authentication_token: NodeId::new_numeric(0, 0),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let client_protocol_version: u32 = 0;
        let request_type = SecurityTokenRequestType::ISSUE;
        let security_mode = MessageSecurityMode::NONE;
        let client_nonce = ByteString::new();
        let requested_lifetime: u32 = 360000;
        OpenSecureChannelRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            client_protocol_version,
            request_type,
            security_mode,
            client_nonce,
            requested_lifetime,
        }
    }
}

impl OpenSecureChannelRequest {
    pub fn build(
        sender_certificate: &ByteString,
        receiver_certificate_thumbprint: &ByteString,
        security_mode: u32,
        security_policy:&SecurityPolicy,
        requested_lifetime: u32
    ) -> Self {
        let message_header: MessageHeader = Default::default();
        let security_header: AsymmetricSecurityHeader ;
        if security_mode==MessageSecurityMode::NONE{
            let sender_certificate = ByteString::new();
            let receiver_certificate_thumbprint = ByteString::new();
            security_header = AsymmetricSecurityHeader::new(
                SecurityPolicyUri::None,
                sender_certificate,
                receiver_certificate_thumbprint,
            );
        }else{
            let sender_certificate = sender_certificate.clone();
            let receiver_certificate_thumbprint = receiver_certificate_thumbprint.clone();
            security_header = AsymmetricSecurityHeader::new(
                security_policy.policy_uri,
                sender_certificate,
                receiver_certificate_thumbprint,
            );
        }
        
        //those fields are uptdate before sending
        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 0,
            request_id: 0,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 446);
        let request_header = RequestHeader {
            authentication_token: NodeId::new_numeric(0, 0),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let client_protocol_version: u32 = 0;
        let request_type = SecurityTokenRequestType::ISSUE;
        let security_mode = security_mode;
        let client_nonce = crate::crypto::random::byte_string(security_policy.secure_channel_nonce_length);
        OpenSecureChannelRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            client_protocol_version,
            request_type,
            security_mode,
            client_nonce,
            requested_lifetime,
        }
    }
}



pub struct SecurityTokenRequestType(u32);
impl SecurityTokenRequestType {
    pub const ISSUE: u32 = 0;
    pub const RENEW: u32 = 1;
}

#[derive(Debug,Serialize,Deserialize)]
pub struct OpenSecureChannelResponse {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: AsymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) server_protocol_version: u32,
    pub(crate) secure_channel_id: u32,
    pub(crate) token_id: u32,
    pub(crate) created_at: DateTime,
    pub(crate) revised_lifetime: u32,
    pub(crate) server_nonce: ByteString,
}