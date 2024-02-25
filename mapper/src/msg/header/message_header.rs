use derive_macro::{Deserialize, Serialize};

use crate::{msg::message_type::MessageType, Deserialize, Serialize, MapperResult};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct MessageHeader {
    pub(crate) message_type: MessageType,
    pub(crate) is_final: u8,
    pub(crate) message_size: u32,
    pub(crate) secure_channel_id: u32,
}

impl Default for MessageHeader {
    //the default size is the size of a default opensecure channel request with None as security policy
    fn default() -> Self {
        MessageHeader {
            message_type: MessageType::OPN,
            is_final: MessageHeader::convert_final_to_u8(b"F"),
            message_size: 133,
            secure_channel_id: 0,
        }
    }
}

impl MessageHeader {
    pub fn set_message_type(&mut self, message_type: MessageType) {
        self.message_type = message_type;
    }
    pub fn set_message_size(&mut self, message_size: u32) {
        self.message_size = message_size;
    }
    pub fn set_secure_channel_id(&mut self, secure_channel_id: u32) {
        self.secure_channel_id = secure_channel_id;
    }
    pub fn get_message_type(&mut self) -> &MessageType {
        &self.message_type
    }
    pub fn get_message_size(&mut self) -> &u32 {
        &self.message_size
    }
    pub fn get_secure_channel_id(&mut self) -> &u32 {
        &self.secure_channel_id
    }
    pub fn is_final(value: &[u8]) -> &'static [u8; 1] {
        match value {
            b"F" => b"F",
            b"A" => b"A",
            b"C" => b"C",
            _ => panic!("unexpected message type\n"),
        }
    }
    fn convert_final_to_u8(final_field: &[u8; 1]) -> u8 {
        //         C An intermediate chunk.
        //         F The final chunk.
        //         A The final chunk (used when an error occurred and the Message is aborted).
        match final_field {
            b"F" => 70,
            b"C" => 67,
            b"A" => 65,
            _ => 70,
        }
    }
    /// Build the message header but the size need to be set before sending
    pub fn build(message_type: MessageType, is_final: u8, secure_channel_id: u32) -> MessageHeader {
        MessageHeader {
            message_type,
            is_final,
            message_size: 0,
            secure_channel_id,
        }
    }
}
