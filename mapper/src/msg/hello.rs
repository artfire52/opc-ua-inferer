use std::default::Default;

use super::super::uatypes::string::UaString;
use crate::encoding_prelude::*;

use super::header::connection_message_header::ConnectionMessageHeader;

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct HelloMessage {
    pub(crate) connection_header: ConnectionMessageHeader,
    pub(crate) protocol_version: u32,
    pub(crate) receiver_buffer_size: u32,
    pub(crate) send_buffer_size: u32,
    pub(crate) max_msg_size: u32,
    pub(crate) max_chunk_count: u32,
    pub(crate) endpoint_url: UaString,
}

impl Default for HelloMessage {
    fn default() -> Self {
        HelloMessage {
            connection_header: Default::default(),
            protocol_version: 0,
            receiver_buffer_size: 65536,
            send_buffer_size: 65536,
            max_msg_size: 16777216,
            max_chunk_count: 5000,
            endpoint_url: UaString::from("opc.tcp://localhost:4840"),
        }
    }
}

impl HelloMessage {
    pub fn build(endpoint_url:&UaString) -> Self {
        HelloMessage {
            connection_header: Default::default(),
            protocol_version: 0,
            receiver_buffer_size: 65536,
            send_buffer_size: 65536,
            max_msg_size: 16777216,
            max_chunk_count: 5000,
            endpoint_url: endpoint_url.clone(),
        }
    }
}
