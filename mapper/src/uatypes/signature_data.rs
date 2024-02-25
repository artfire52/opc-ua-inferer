use crate::{encoding_prelude::*, crypto::{security_policy::SecurityPolicy, }};

use super::{string::UaString, byte_string::ByteString};
use crate::crypto::policy_prelude::*;
#[derive(Debug, Serialize, Deserialize)]
pub (crate) struct SignatureData{
    pub (crate)algorithm: UaString,
    pub (crate) signature: ByteString,
}

impl SignatureData{
    pub(crate) fn new()->SignatureData{
        SignatureData{
            algorithm: UaString::new(),
            signature: ByteString::new(),
        }
    }
    pub(crate) fn from(security_policy: &mut SecurityPolicy,certificate:&ByteString,server_nonce:Option<&ByteString>)->SignatureData{
        
        let mut to_sign=match &certificate.value{
            Some(v)=> v.clone(),
            None=> return SignatureData::new(),

        };
        match server_nonce{
            Some(nonce)=>if let Some(data)=&nonce.value { to_sign.extend_from_slice(data);},
            None=> {},

        };
        
        let signer=match &mut security_policy.certificate_signature_algorithm{
            Some(v)=> v,
            None=> return SignatureData::new(),
        };
    
        let signature=signer.sign(&to_sign);


        SignatureData{
            algorithm: UaString::from(SecurityPolicyUri::get_asym_signing_algo_uri(security_policy.policy_uri)),
            signature: ByteString::from(signature),
        }
    }
}