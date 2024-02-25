use crate::encoding_prelude::*;

use super::{string::UaString, byte_string::ByteString};

#[derive(Debug, PartialEq,Deserialize,Serialize,Clone)]
pub struct UserTokenPolicy {
    pub(crate) policy_id: UaString,
    pub(crate) token_type: u32,
    pub(crate) issued_token_type: UaString,
    pub(crate) issuer_endpoint_url: UaString,
    pub(crate) security_policy_uri: UaString,
}

#[derive(Debug, PartialEq,Deserialize,Serialize)]
pub struct UserNameIdentityToken {
    pub(crate) policy_id: UaString,
    pub(crate) username: UaString,
    pub(crate) password: ByteString,
    pub(crate) encryption_algorithm: UaString,
}

#[derive(Debug, PartialEq,Deserialize,Serialize)]
pub struct X509IdentityToken {
    pub(crate) policy_id: UaString,
    pub(crate) certificate: ByteString,
}

#[derive(Debug, PartialEq,Deserialize,Serialize)]
pub struct AnonymousIdentityToken {
    pub(crate) policy_id: UaString,
}






pub struct UserTokenType;
impl UserTokenType {
    pub const ANONYMOUS: u32 = 0;
    pub const USERNAME: u32 = 1;
    pub const CERTIFICATE: u32 = 2;
    pub const ISSUEDTOKEN: u32 = 3;
}
