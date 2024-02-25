use derive_macro::{Deserialize, Serialize};

// use super::message_type::MessageType;
use crate::{msg::message_type::MessageType, Deserialize, Serialize,MapperResult};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct ConnectionMessageHeader {
    pub(crate) message_type: MessageType,
    pub(crate) _reserved: u8,
    pub(crate) message_size: u32,
}

impl Default for ConnectionMessageHeader {
    //the default size is the size of a default hello message
    fn default() -> Self {
        ConnectionMessageHeader {
            message_type: MessageType::HEL,
            _reserved: ConnectionMessageHeader::convert_final_to_u8(b"F"),
            message_size: 56,
        }
    }
}

impl ConnectionMessageHeader {
    pub fn set_message_type(&mut self, message_type: MessageType) {
        self.message_type = message_type;
    }
    pub fn set_message_size(&mut self, message_size: u32) {
        self.message_size = message_size;
    }
    pub fn get_message_type(&mut self) -> &MessageType {
        &self.message_type
    }
    pub fn get_message_size(&mut self) -> &u32 {
        &self.message_size
    }

    pub fn convert_final_to_u8(final_field: &[u8; 1]) -> u8 {
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
}

impl ConnectionMessageHeader {
    pub fn new(
        message_type: MessageType,
        _reserved: &[u8; 1],
        message_size: u32,
    ) -> ConnectionMessageHeader {
        ConnectionMessageHeader {
            message_type,
            _reserved: ConnectionMessageHeader::convert_final_to_u8(_reserved),
            message_size,
        }
    }
    /// To build the connection header the size has to be sent before sending but not at initialization
    pub fn build(message_type: MessageType, _reserved: &[u8; 1]) -> ConnectionMessageHeader {
        ConnectionMessageHeader {
            message_type,
            _reserved: ConnectionMessageHeader::convert_final_to_u8(_reserved),
            message_size: 0,
        }
    }

    pub fn is_final(value: &[u8]) -> &'static [u8; 1] {
        match value {
            b"F" => b"F",
            b"A" => b"A",
            b"C" => b"C",
            _ => panic!("unexpected message type\n"),
        }
    }
}
