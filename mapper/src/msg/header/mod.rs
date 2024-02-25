//! this module is usefull to handle the several headers.
pub mod additional_header;
pub mod connection_message_header;
pub mod footer;
pub mod message_header;
pub mod request_header;
pub mod response_header;
pub mod security_header;
pub mod sequence_header;

pub(crate) mod prelude{
    pub(crate) use crate::msg::header::sequence_header::*;
    pub(crate) use crate::msg::header::response_header::*;
    pub(crate) use crate::msg::header::connection_message_header::*;
    pub(crate) use crate::msg::header::message_header::*;
    pub(crate) use crate::msg::header::security_header::*;
    pub(crate) use crate::msg::header::request_header::*;
}
