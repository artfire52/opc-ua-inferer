use super:: message_type::MessageType;
use super::header::prelude::*;
use crate::uatypes::prelude::*;
use crate::encoding_prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct CloseSecureChannelRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
}

impl CloseSecureChannelRequest {
    pub (crate) fn build() -> Self {
        let mut message_header: MessageHeader = Default::default();
        message_header.secure_channel_id = 0;
        message_header.message_type = MessageType::CLO;
        let mut security_header: SymmetricSecurityHeader = Default::default();
        security_header.token_id = 0;
        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 3,
            request_id: 3,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 452);
        let request_header = RequestHeader {
            authentication_token: NodeId::new_numeric(0, 0),
            timestamp: DateTime::new_now(),
            request_handle: 2,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };

        CloseSecureChannelRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
        }
    }
}
