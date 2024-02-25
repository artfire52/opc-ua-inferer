use crate::crypto::pkey::{PublicKey, PrivateKey};
use crate::crypto::security_policy::{SecurityPolicy};
use crate::uatypes::prelude::*;
use crate::encoding_prelude::*;

use super::header::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSessionRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) client_signature: SignatureData,
    pub(crate) future_use: i32,
    pub(crate) locale_ids: Vec::<UaString>,
    pub(crate) user_identity_token: ExtensionObject,
    pub(crate) user_token_signature: SignatureData,
}

impl ActiveSessionRequest {
    pub fn build(server_public_key: &PublicKey,private_key:&PrivateKey,server_nonce: &ByteString,server_cert: &ByteString,user_cert: &ByteString,session_node_id:&NodeId,security_policy: &mut SecurityPolicy,policy_token_uri:&str,policy_id:&UaString,is_anon:bool,is_user:bool,is_cert:bool) -> Self {
        let message_header = MessageHeader {
            message_type: super::message_type::MessageType::MSG,
            is_final: b'F',
            message_size: 0,
            secure_channel_id: 0,
        };
        let security_header: SymmetricSecurityHeader = SymmetricSecurityHeader::default();

        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 0,
            request_id: 0,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 467);
        let request_header = RequestHeader {
            authentication_token: session_node_id.clone(),
            timestamp: DateTime::new_now(),
            request_handle: 100,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let client_signature=SignatureData::from(security_policy,server_cert,Some(server_nonce));
        let future_use=0;
        let locale_ids = vec![UaString::from("en-US")];
        let user_identity_token : ExtensionObject;
        match (is_anon,is_user,is_cert){
            (true,_,_)=>user_identity_token = ExtensionObject::anon(policy_id),
            (false,_,true)=>user_identity_token =ExtensionObject::new_user_cert(policy_id,user_cert),
            (false,true,false)=>user_identity_token = ExtensionObject::new_user("user1","password",policy_token_uri,policy_id,server_public_key,private_key,server_nonce),
            (false,false,false)=> user_identity_token = ExtensionObject::new_user("atv2","letmein",policy_token_uri,policy_id,server_public_key,private_key,server_nonce),
        }
        let user_token_signature :SignatureData;
        if is_cert{
            let mut security_policy_token: SecurityPolicy = SecurityPolicy::new(policy_token_uri);
            security_policy_token.set_asym(&private_key, &server_public_key);
            user_token_signature =SignatureData::from(&mut security_policy_token,server_cert,Some(server_nonce));
        }else{
            user_token_signature =SignatureData::new();
        }
        ActiveSessionRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            client_signature,
            future_use,
            locale_ids,
            user_identity_token,
            user_token_signature,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSessionResponse{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) server_nonce: ByteString,
    pub(crate) results: Vec::<StatusCode>,
    pub(crate) diagnosti_info: Vec::<DiagnosticInfo>,
}
