use crate::encoding_prelude::*;

use super::header::prelude::*;
use crate::uatypes::prelude::*;
use crate::crypto::security_policy::SecurityPolicyUri;

#[derive(Debug,Serialize,Deserialize)]
pub struct NullSize {
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

impl NullSize {
    pub fn build(
        sender_certificate: &ByteString,
        receiver_certificate_thumbprint: &ByteString,
        security_mode: u32,
        requested_lifetime: u32
    ) -> Self {
        let mut message_header: MessageHeader = Default::default();
        message_header.message_size=0;//u32::MAX;
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
                SecurityPolicyUri::Basic256Sha256,
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
        let client_nonce = crate::crypto::random::byte_string(32);
        NullSize {
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
