use super::{node_id::NodeId, string::UaString, byte_string::ByteString, date_time::DateTime};


pub struct RsaEncryptedSecret{
    pub(crate) type_id:NodeId,
    pub(crate) endcoding_mask:u8,//must be 1 all the time
    pub(crate) length: u32,
    pub(crate) security_policy_uri: UaString,
    pub(crate) certificate: ByteString,
    pub(crate) signing_time: DateTime,
    pub(crate) key_data_length: u16,
    pub(crate) signing_key: ByteString,
    pub(crate) encryption_key: ByteString,
    pub(crate) initialization_vector: ByteString,
    pub(crate) nonce: ByteString,
    pub(crate) secret: ByteString,
    //We ignore the padding and the signature. We let functions add it. We does not require to know a thing about padding
}