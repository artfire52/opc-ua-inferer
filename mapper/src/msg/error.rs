use std::default::Default;

use super::header::connection_message_header::ConnectionMessageHeader;
use crate::msg::message_type::MessageType;
use crate::uatypes::prelude::*;
use crate::encoding_prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub (crate) struct ErrorMessage {
    pub(crate) connection_header: ConnectionMessageHeader,
    pub(crate) error: StatusCode,
    pub(crate) reason: UaString,
}

impl Default for ErrorMessage {
    fn default() -> Self {
        ErrorMessage {
            connection_header: ErrorMessage::default_connection_header(),
            error: StatusCode::new(StatusCode::Bad),
            reason: UaString::from("nope"),
        }
    }
}

impl ErrorMessage {
    pub (self) fn default_connection_header() -> ConnectionMessageHeader {
        ConnectionMessageHeader::new(MessageType::ERR, b"F", 0)
    }
}
