use derive_macro::{Deserialize, Serialize};

use crate::{
    uatypes::{byte_string::ByteString, string::UaString},
    Deserialize, Serialize, MapperResult
};
#[derive(Debug, Serialize)]
pub (crate)  enum SecurityHeader {
    Asymmetric(AsymmetricSecurityHeader),
    Symmetric(SymmetricSecurityHeader),
}

//length fields are not present because there are contained in ByteString type and UaString type
#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct AsymmetricSecurityHeader {
    pub(crate) security_policy_uri: UaString,
    pub(crate) sender_certificate: ByteString,
    pub(crate) receiver_certificate_thumbprint: ByteString,
}

#[derive(Debug, Serialize, Deserialize)]
pub (crate)  struct SymmetricSecurityHeader {
    pub(crate) token_id: u32,
}

impl Default for AsymmetricSecurityHeader {
    fn default() -> Self {
        AsymmetricSecurityHeader {
            security_policy_uri: UaString::from("http://opcfoundation.org/UA/SecurityPolicy#None"),
            sender_certificate: ByteString::new(),
            receiver_certificate_thumbprint: ByteString::new(),
        }
    }
}

impl AsymmetricSecurityHeader {
    pub fn new(
        security_policy_uri: &str,
        sender_certificate: ByteString,
        receiver_certificate_thumbprint: ByteString,
    ) -> Self {
        AsymmetricSecurityHeader {
            security_policy_uri: UaString::from(security_policy_uri),
            sender_certificate,
            receiver_certificate_thumbprint,
        }
    }
}

impl Default for SymmetricSecurityHeader {
    fn default() -> Self {
        SymmetricSecurityHeader { token_id: 0 }
    }
}

