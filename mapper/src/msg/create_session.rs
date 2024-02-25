use crate::uatypes::prelude::*;
use crate::encoding_prelude::*;
use std::num::ParseIntError;

use super::header::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) client_description: ApplicationDescription,
    pub(crate) server_uri: UaString,
    pub(crate) endpoint_url: UaString,
    pub(crate) session_name: UaString,
    pub(crate) client_nonce: ByteString,
    pub(crate) client_certificate: ByteString,
    pub(crate) requested_session_timeout: f64,
    pub(crate) max_response_message_size: u32,
}

impl CreateSessionRequest {
    pub fn build(endpoint_url:&UaString,sender_certificate: &ByteString,session_timeout: f64) -> Self {
        // let message_header: MessageHeader= Default::default();
        let message_header = MessageHeader {
            message_type: super::message_type::MessageType::MSG,
            is_final: b'F',
            message_size: 0,
            secure_channel_id: 0,
        };
        let security_header: SymmetricSecurityHeader = SymmetricSecurityHeader::default();

        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 2,
            request_id: 2,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 461);
        let request_header = RequestHeader {
            authentication_token: NodeId::new_numeric(0, 0),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let (_,client_description)=ApplicationDescription::deserialize(&CreateSessionRequest::decode_hex("2e00000075726e3a6172746875722d636f6d70757465723a556e69666965644175746f6d6174696f6e3a55614578706572741e00000075726e3a556e69666965644175746f6d6174696f6e3a556145787065727402180000005561457870657274406172746875722d636f6d707574657201000000ffffffffffffffff00000000").unwrap().to_vec()).unwrap();
        let server_uri = UaString::new();
        let endpoint_url = endpoint_url.clone();
        let session_name = UaString::from("urn:arthur-computer:UnifiedAutomation:UaExpert");
        let client_nonce = crate::crypto::random::byte_string(32);
        let client_certificate = sender_certificate.clone();
        let requested_session_timeout = session_timeout;
        let max_response_message_size = 16777216;
        CreateSessionRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            client_description,
            server_uri,
            endpoint_url,
            session_name,
            client_nonce,
            client_certificate,
            requested_session_timeout,
            max_response_message_size,
        }
    }
    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionResponse{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) session_id: NodeId,
    pub(crate) authentication_token: NodeId,
    pub(crate) revised_session_timeout: f64,
    pub(crate) server_nonce: ByteString,
    pub(crate) server_certificate: ByteString,
    pub(crate) endpoints: Vec::<EndpointDescription>,
    pub(crate) deprecated: i32,
    pub(crate) server_signatures: SignatureData,
    pub(crate) max_response_message_size: u32,
}
