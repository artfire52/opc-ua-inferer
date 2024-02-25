use crate::encoding_prelude::*;

use super::{
    application_description::ApplicationDescription, byte_string::ByteString, string::UaString,
    user_identity_token::UserTokenPolicy,
};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct EndpointDescription {
    pub (crate) endpoint_url: UaString,
    pub (crate) server: ApplicationDescription,
    pub (crate) server_certificate: ByteString,
    pub (crate) security_mode: u32,
    pub (crate) security_policy_uri: UaString,
    pub (crate) user_identity_tokens: Vec::<UserTokenPolicy>,
    pub (crate) transport_profile_uri: UaString,
    pub (crate) security_level: u8,
}
