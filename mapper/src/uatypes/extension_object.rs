use crate::{crypto::{security_policy::{SecurityPolicyUri, SecurityPolicy}, pkey::PrivateKey}};
use crate::encoding_prelude::*;
use crate::crypto::pkey::PublicKey;
use super::prelude::*;

#[derive(Debug,Clone)]
pub struct ExtensionObject {
    pub(crate) type_id: NodeId,
    pub(crate) encoding: u8,
    pub(crate) body: ExtensionObjectBody,
}

//Directly encoded as ByteString
#[derive(Debug,Clone)]
pub enum ExtensionObjectBody {
    None,
    ByteString(ByteString),
    XmlElement(XmlElement),
}

pub struct EncodingValue;
impl EncodingValue {
    pub const NO_BODY: u8 = 0x00;
    pub const BYTE_STRING: u8 = 0x01;
    pub const XML_ELEMENT: u8 = 0x02;
}
impl ExtensionObject{
    pub(crate) fn new_user(user:&str,passwd:&str,security_policy_uri:&str,policy_id:&UaString, server_public_key:&PublicKey,private_key:&PrivateKey,server_nonce:&ByteString )-> Self{
        
        let mut body=UserNameIdentityToken{
            policy_id: policy_id.clone(),
            username: UaString::from(user),
            password: ByteString::from_str(passwd),
            encryption_algorithm: UaString::from(SecurityPolicyUri::get_asym_enc_algo_uri(security_policy_uri)),

        };
        if security_policy_uri!= SecurityPolicyUri::None{
            let mut security_policy: SecurityPolicy = SecurityPolicy::new(security_policy_uri);
            security_policy.set_asym(&private_key, &server_public_key);
            let encrypter=&mut security_policy.asymmetric_encryption.unwrap();
            body.password.append_byte_string(server_nonce);
            let to_encrypt=body.password.serialize();
            let encrypted_passwd= encrypter.encrypt(&to_encrypt);
            body.password=ByteString::from(encrypted_passwd);
        }else{
            
        }
        ExtensionObject {
            type_id: NodeId::new_numeric(0, 324),
            encoding: EncodingValue::BYTE_STRING,
            body:ExtensionObjectBody::ByteString(ByteString::from(body.serialize())),
        }
    }
    pub(crate) fn anon(anonymous_policy_id:&UaString)-> Self{
        
        let body=AnonymousIdentityToken{
            policy_id: anonymous_policy_id.clone(),

        };

        ExtensionObject {
            type_id: NodeId::new_numeric(0, 321),
            encoding: EncodingValue::BYTE_STRING,
            body:ExtensionObjectBody::ByteString(ByteString::from(body.serialize())),
        }
    }
    pub(crate) fn new_user_cert(policy_id:&UaString,user_certificate:&ByteString )-> Self{
        
        let body=X509IdentityToken{
            policy_id: policy_id.clone(),
            certificate: user_certificate.clone(),

        };
        ExtensionObject {
            type_id: NodeId::new_numeric(0, 327),
            encoding: EncodingValue::BYTE_STRING,
            body:ExtensionObjectBody::ByteString(ByteString::from(body.serialize())),
        }
    }

}
impl Serialize for ExtensionObject {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.type_id.serialize();
        match &self.body {
            ExtensionObjectBody::None => result.push(EncodingValue::NO_BODY),

            ExtensionObjectBody::ByteString(b) => {
                result.push(EncodingValue::BYTE_STRING);
                result.extend_from_slice(&b.serialize());
            }

            ExtensionObjectBody::XmlElement(x) => {
                result.push(EncodingValue::BYTE_STRING);
                result.extend_from_slice(&x.serialize());
            }
        }
        result
    }
}

impl Deserialize for ExtensionObject {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, type_id) = NodeId::deserialize(data)?;
        let (data, encoding) = u8::deserialize(data)?;
        match encoding {
            EncodingValue::NO_BODY => Ok((
                data,
                ExtensionObject {
                    type_id,
                    encoding,
                    body: ExtensionObjectBody::None,
                },
            )),
            EncodingValue::BYTE_STRING => {
                let (data, body) = ByteString::deserialize(data)?;
                Ok((
                    data,
                    ExtensionObject {
                        type_id,
                        encoding,
                        body: ExtensionObjectBody::ByteString(ByteString::from(body.serialize())),
                    },
                ))
            }
            EncodingValue::XML_ELEMENT => {
                let (data, body) = XmlElement::deserialize(data)?;
                Ok((
                    data,
                    ExtensionObject {
                        type_id,
                        encoding,
                        body: ExtensionObjectBody::XmlElement(body),
                    },
                ))
            }
            _ => Ok((
                data,
                ExtensionObject {
                    type_id,
                    encoding,
                    body: ExtensionObjectBody::None,
                },
            )),
        }
    }
}
