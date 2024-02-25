// # uatypes
//
//! `uatypes` is the part of the project that contain
//! all data type used in our mapper. Some function are provided
//! such as serialization, deserialization etc ...
//! Not all of them are built-in types

pub mod application_description;
pub mod array;
pub mod basic_type;
pub mod byte_string;
pub mod data_value;
pub mod date_time;
pub mod diagnostic_info;
pub mod endpoint_description;
pub mod expanded_node_id;
pub mod extension_object;
pub mod guid;
pub mod localized_text;
pub mod node_id;
pub mod qualified_name;
pub mod security_mode;
pub mod status_code;
pub mod string;
pub mod signature_data;
pub mod encrypted_secret;
pub mod read_value_id;
#[cfg(test)]
mod tests;
pub mod user_identity_token;
pub mod variant;
pub mod xml_element;
pub mod attribute_id;
pub mod write_value;
pub mod data_type_id;


pub mod prelude {
    pub(crate) use crate::uatypes::application_description::*;
    pub(crate) use crate::uatypes::diagnostic_info::*;
    pub(crate) use crate::uatypes::extension_object::*;
    pub(crate) use crate::uatypes::byte_string::*;
    pub(crate) use crate::uatypes::data_value::*;
    pub(crate) use crate::uatypes::date_time::*;
    pub(crate) use crate::uatypes::endpoint_description::*;
    pub(crate) use crate::uatypes::guid::*;
    pub(crate) use crate::uatypes::node_id::*;
    pub(crate) use crate::uatypes::expanded_node_id::*;
    pub(crate) use crate::uatypes::qualified_name::*;
    pub(crate) use crate::uatypes::security_mode::*;
    pub(crate) use crate::uatypes::signature_data::*;
    pub(crate) use crate::uatypes::status_code::*;
    pub(crate) use crate::uatypes::string::*;
    pub(crate) use crate::uatypes::read_value_id::*;
    pub(crate) use crate::uatypes::xml_element::*;
    pub(crate) use crate::uatypes::localized_text::*;
    pub(crate) use crate::uatypes::user_identity_token::*;
    pub(crate) use crate::uatypes::attribute_id::*;
    pub(crate) use crate::uatypes::write_value::*;
    pub(crate) use crate::uatypes::variant::*;
    pub(crate) use crate::uatypes::data_type_id::*;
    pub(crate) use super::ToVariant;
}

pub trait ToVariant{
    fn to_variant(&self)->prelude::Variant;
}