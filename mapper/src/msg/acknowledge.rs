use crate::encoding_prelude::*;
use crate::msg::message_type::MessageType;
use crate::msg::header::prelude::*;
use std::default::Default;

#[derive(Debug,Deserialize,Serialize)]
pub (crate)  struct AckowledgeMessage {
    pub(crate) connection_header: ConnectionMessageHeader,
    pub(crate) protocol_version: u32,
    pub(crate) receiver_buffer_size: u32,
    pub(crate) send_buffer_size: u32,
    pub(crate) max_msg_size: u32,
    pub(crate) max_chunk_count: u32,
}

impl Default for AckowledgeMessage {
    fn default() -> Self {
        AckowledgeMessage {
            connection_header: AckowledgeMessage::default_connection_header(),
            protocol_version: 0,
            receiver_buffer_size: 65536,
            send_buffer_size: 65536,
            max_msg_size: 16777216,
            max_chunk_count: 5000,
        }
    }
}



impl AckowledgeMessage {
    pub fn default_connection_header() -> ConnectionMessageHeader {
        ConnectionMessageHeader {
            message_type: MessageType::ACK,
            _reserved: ConnectionMessageHeader::convert_final_to_u8(b"F"),
            message_size: 0,
        }
    }
}
