use crate::uatypes::prelude::*;
use crate::encoding_prelude::*;
use std::num::ParseIntError;

use super::header::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseSessionRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) delete_subscription: bool,
   
}

impl CloseSessionRequest {
    pub fn build(authentication_token:&NodeId) -> Self {
        // let message_header: MessageHeader= Default::default();
        let message_header = MessageHeader {
            message_type: super::message_type::MessageType::MSG,
            is_final: b'F',
            message_size: 0,
            secure_channel_id: 0,
        };
        let security_header: SymmetricSecurityHeader = SymmetricSecurityHeader::default();

        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 26,
            request_id: 26,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 473);
        let request_header = RequestHeader {
            authentication_token: authentication_token.clone(),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        CloseSessionRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            delete_subscription : false,
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
pub struct CloseSessionResponse{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
}
