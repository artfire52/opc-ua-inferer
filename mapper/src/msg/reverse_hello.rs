use std::default::Default;
use super::header::connection_message_header::ConnectionMessageHeader;
use crate::msg::message_type::MessageType;
use crate::uatypes::string::UaString;
use crate::encoding_prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct RevHelloMessage {
    pub(crate) connection_header: ConnectionMessageHeader,
    pub(crate) server_uri: UaString,
    pub(crate) endpoint_url: UaString,
}

impl Default for RevHelloMessage {
    fn default() -> Self {
        RevHelloMessage {
            connection_header: RevHelloMessage::default_connection_header(),
            server_uri: UaString::from("urn:default"),
            endpoint_url: UaString::from("ocp.tcp://localhost:4840"),
        }
    }
}

impl RevHelloMessage {
    pub fn default_connection_header() -> ConnectionMessageHeader {
        ConnectionMessageHeader::new(MessageType::RHE, b"F", 0)
    }
}
