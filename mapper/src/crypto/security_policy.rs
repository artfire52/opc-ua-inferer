use openssl::hash::MessageDigest;

use crate::MapperResult;
use crate::error::{MapperError, MapperErrorKind};
use crate::uatypes::byte_string::ByteString;

use super::encryption::{
    AsymmetricEncryptionAlgorithm, EncrypterDecrypter, SymCipher, SymmetricEncryptionAlgorithm,
};
use super::hash;
use super::pkey::{PrivateKey, PublicKey};
use super::signature::{
    AsymmetricSignatureAlgorithm, AsymmetricSigner, SymmetricSignatureAlgorithm, SymmetricSigner,
};

/// Contain the element required to comply with a security policy.
/// In other words, it contain the signer, encrypter ...
pub struct SecurityPolicy<'a> {
    pub(crate) symmetric_signature_algorithm_client: Option<SymmetricSigner>,
    pub(crate) symmetric_encryption_client: Option<SymCipher>,
    pub(crate) policy_uri: &'a str,
    pub(crate) symmetric_signature_algorithm_server: Option<SymmetricSigner>,
    pub(crate) symmetric_encryption_server: Option<SymCipher>,
    pub(crate) symmetric_encryption_algo: Option<SymmetricEncryptionAlgorithm>,
    pub(crate) symmetric_signature_algo: Option<SymmetricSignatureAlgorithm>,
    pub(crate) asymmetric_signature_algorithm: Option<AsymmetricSigner<'a>>,
    pub(crate) asymmetric_encryption: Option<EncrypterDecrypter<'a>>,
    pub(crate) min_asymmetric_key_length: usize,
    pub(crate) max_asymmetric_key_length: usize,
    pub(crate) key_derivation_algorithm: Option<KeyDerivationAlgorithm>,
    pub(crate) derived_signature_key_length: usize,
    pub(crate) certificate_signature_algorithm: Option<AsymmetricSigner<'a>>,
    pub(crate) certificate_key_algorithm: Option<CertificateKeyAlgorithm>,
    pub(crate) ephemeral_key_algorithm: Option<EphemeralKeyAlgorithm>,
    pub(crate) secure_channel_nonce_length: usize,
    pub(crate) initialization_vector_length: usize,
    pub(crate) symmetric_signature_length: usize,
    pub(crate) legacy_sequence_numbers: bool,
}

type DerivationFunction =
    fn(message_digest: MessageDigest, secret: &[u8], seed: &[u8], length: usize) -> Vec<u8>;
pub(crate) struct KeyDerivationAlgorithm {
    pub(crate) derive_function: DerivationFunction,
    pub(crate) message_digest: MessageDigest,
}

impl KeyDerivationAlgorithm {
    pub fn new(
        derive_function: DerivationFunction,
        message_digest: MessageDigest,
    ) -> KeyDerivationAlgorithm {
        KeyDerivationAlgorithm {
            derive_function,
            message_digest,
        }
    }

    pub fn derive_key(&self, secret: &[u8], seed: &[u8], length: usize) -> Vec<u8> {
        (self.derive_function)(self.message_digest, secret, seed, length)
    }
}
pub(crate) enum CertificateKeyAlgorithm {
    RSA,
}

pub(crate) enum EphemeralKeyAlgorithm {
    RSA,
}

///security policy uri defined in <https://profiles.opcfoundation.org/profilefolder/474>
pub struct SecurityPolicyUri;
#[allow(non_upper_case_globals)]
//we conserve the syntax provided in the documentation
#[allow(dead_code)]
impl SecurityPolicyUri {
    pub const None: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#None";
    pub const Aes128_Sha256_RsaOaep: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#Aes128_Sha256_RsaOaep";
    pub const Basic256Sha256: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#Basic256Sha256";
    pub const Aes256_Sha256_RsaPss: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#Aes256_Sha256_RsaPss";
    pub const ECC_curve25519: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_curve25519";
    pub const ECC_nistP256: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_nistP256";
    pub const ECC_nistP384: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_nistP384";
    pub const ECC_brainpoolP256r1: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_brainpoolP256r1";
    pub const ECC_brainpoolP384r1: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_brainpoolP384r1";
    pub const ECC_curve448: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#ECC_curve448";
    pub const Basic128Rsa15: &'static str =
        "http://opcfoundation.org/UA/SecurityPolicy#Basic128Rsa15";
    pub const Basic256: &'static str = "http://opcfoundation.org/UA/SecurityPolicy#Basic256";

    pub fn get_asym_enc_algo_uri(security_policy_uri:&str)->&str{
        match security_policy_uri{
            SecurityPolicyUri::None=>{""},
            SecurityPolicyUri::Aes128_Sha256_RsaOaep=>{"http://www.w3.org/2001/04/xmlenc#rsa-oaep"},
            SecurityPolicyUri::Basic256Sha256=>{"http://www.w3.org/2001/04/xmlenc#rsa-oaep"},
            // SecurityPolicyUri::ECC_curve25519=>{},
            // SecurityPolicyUri::ECC_nistP256=>{},
            // SecurityPolicyUri::ECC_nistP384=>{},
            // SecurityPolicyUri::ECC_brainpoolP256r1=>{},
            // SecurityPolicyUri::ECC_brainpoolP384r1=>{},
            // SecurityPolicyUri::ECC_curve448=>{},
            SecurityPolicyUri::Basic128Rsa15=>{"http://www.w3.org/2001/04/xmlenc#rsa-1_5"},
            SecurityPolicyUri::Basic256=>{"http://www.w3.org/2001/04/xmlenc#rsa-oaep"},
            _=>{"http://www.w3.org/2001/04/xmlenc#rsa-oaep"},

        }

    }

    pub fn get_asym_signing_algo_uri(security_policy_uri:&str)->&str{
        match security_policy_uri{
            SecurityPolicyUri::None=>{""},
            SecurityPolicyUri::Aes128_Sha256_RsaOaep=>{"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"},
            SecurityPolicyUri::Basic256Sha256=>{"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"},
            // SecurityPolicyUri::ECC_curve25519=>{},
            // SecurityPolicyUri::ECC_nistP256=>{},
            // SecurityPolicyUri::ECC_nistP384=>{},
            // SecurityPolicyUri::ECC_brainpoolP256r1=>{},
            // SecurityPolicyUri::ECC_brainpoolP384r1=>{},
            // SecurityPolicyUri::ECC_curve448=>{},
            SecurityPolicyUri::Basic128Rsa15=>{"http://www.w3.org/2000/09/xmldsig#rsa-sha1"},
            SecurityPolicyUri::Basic256=>{"http://www.w3.org/2000/09/xmldsig#rsa-sha1"},
            _=>{"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"},

        }

    }

    pub fn get_security_policy_uri(uri:&str)->&'static str{
        match uri{
            SecurityPolicyUri::Aes128_Sha256_RsaOaep => SecurityPolicyUri::Aes128_Sha256_RsaOaep,
            SecurityPolicyUri::Basic256Sha256 => SecurityPolicyUri::Basic256Sha256,
            SecurityPolicyUri::Aes256_Sha256_RsaPss => SecurityPolicyUri::Aes256_Sha256_RsaPss,
            SecurityPolicyUri::ECC_curve25519 => SecurityPolicyUri::ECC_curve25519,
            SecurityPolicyUri::ECC_nistP256 => SecurityPolicyUri::ECC_nistP256,
            SecurityPolicyUri::ECC_nistP384 => SecurityPolicyUri::ECC_nistP384,
            SecurityPolicyUri::ECC_brainpoolP256r1 => SecurityPolicyUri::ECC_brainpoolP256r1,
            SecurityPolicyUri::ECC_brainpoolP384r1 => SecurityPolicyUri::ECC_brainpoolP384r1,
            SecurityPolicyUri::ECC_curve448 => SecurityPolicyUri::ECC_curve448,
            SecurityPolicyUri::Basic128Rsa15 => SecurityPolicyUri::Basic128Rsa15,
            SecurityPolicyUri::Basic256 => SecurityPolicyUri::Basic256,
            _=>SecurityPolicyUri::None,
        }
    }
}

//all length are in bits except for the secure channel nonce length
impl<'a> SecurityPolicy<'a> {
    pub fn empty() -> SecurityPolicy<'a> {
        SecurityPolicy {
            policy_uri: SecurityPolicyUri::None,
            symmetric_signature_algorithm_client: None,
            symmetric_encryption_client: None,
            symmetric_signature_algorithm_server: None,
            symmetric_encryption_server: None,
            symmetric_encryption_algo: None,
            symmetric_signature_algo: None,
            asymmetric_signature_algorithm: None,
            asymmetric_encryption: None,
            min_asymmetric_key_length: 0,
            max_asymmetric_key_length: 0,
            key_derivation_algorithm: None,
            derived_signature_key_length: 0,
            certificate_signature_algorithm: None,
            certificate_key_algorithm: None,
            ephemeral_key_algorithm: None,
            secure_channel_nonce_length: 0,
            initialization_vector_length: 0,
            symmetric_signature_length: 0,
            legacy_sequence_numbers: false,
        }
    }

    pub fn new(uri:&'a str) -> SecurityPolicy<'a> {
        match uri {
            SecurityPolicyUri::Basic256Sha256=>{
                let key_derive=KeyDerivationAlgorithm {
                    derive_function: hash::p_hash,
                    message_digest: MessageDigest::sha256(),
                };
                SecurityPolicy::new_custom(uri,
                    SymmetricEncryptionAlgorithm::Aes256CBC,
                    SymmetricSignatureAlgorithm::HmacSha256,
                    2048,4096,
                    key_derive,
                    32,
                    32,
                    CertificateKeyAlgorithm::RSA,
                    16,
                    32,
                )
            },
            SecurityPolicyUri::Aes128_Sha256_RsaOaep=>{
                let key_derive=KeyDerivationAlgorithm {
                    derive_function: hash::p_hash,
                    message_digest: MessageDigest::sha256(),
                };
                SecurityPolicy::new_custom(uri,
                    SymmetricEncryptionAlgorithm::Aes128CBC,
                    SymmetricSignatureAlgorithm::HmacSha256,
                    2048,4096,
                    key_derive,
                    32,
                    32,
                    CertificateKeyAlgorithm::RSA,
                    16,
                    32,
                )
            },
            SecurityPolicyUri::Basic128Rsa15=>{
                let key_derive=KeyDerivationAlgorithm {
                    derive_function: hash::p_hash,
                    message_digest: MessageDigest::sha1(),
                };
                SecurityPolicy::new_custom(uri,
                    SymmetricEncryptionAlgorithm::Aes128CBC,
                    SymmetricSignatureAlgorithm::HmacSha1,
                    1024,2048,
                    key_derive,
                    16,
                    16,
                    CertificateKeyAlgorithm::RSA,
                    16,
                    20,
                )
            },
            _=> SecurityPolicy::empty(),
        }
    }
    

    fn new_custom(uri:&'a str,sym_enc_algo:SymmetricEncryptionAlgorithm,sym_sign_algo:SymmetricSignatureAlgorithm,
            min_asymmetric_key_length:usize,max_asymmetric_key_length:usize,key_deriv:KeyDerivationAlgorithm,
            derived_signature_key_length:usize,secure_channel_nonce_length:usize,certificate_key_algorithm:CertificateKeyAlgorithm,
            initialization_vector_length:usize,symmetric_signature_length:usize) -> SecurityPolicy<'a> {
        SecurityPolicy {
            policy_uri: uri,
            symmetric_signature_algorithm_client: None,
            symmetric_encryption_client: None,
            symmetric_signature_algorithm_server: None,
            symmetric_encryption_server: None,
            symmetric_encryption_algo: Some(sym_enc_algo),
            symmetric_signature_algo: Some(sym_sign_algo),
            asymmetric_signature_algorithm: None,
            asymmetric_encryption: None,
            min_asymmetric_key_length,
            max_asymmetric_key_length,
            key_derivation_algorithm: Some(key_deriv),
            derived_signature_key_length,
            certificate_signature_algorithm: None,
            certificate_key_algorithm: Some(certificate_key_algorithm),
            ephemeral_key_algorithm: None,
            secure_channel_nonce_length,
            initialization_vector_length,
            symmetric_signature_length,
            legacy_sequence_numbers: false,
        }
    }

    pub fn set_asym(&mut self,key: &'a PrivateKey,pubkey: &'a PublicKey) {
        match self.policy_uri {
            SecurityPolicyUri::Basic256Sha256=>self.set_asym_custom(key,pubkey,
                AsymmetricSignatureAlgorithm::RsaPKCS15Sha256,
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1,
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256),

            SecurityPolicyUri::Aes128_Sha256_RsaOaep=>self.set_asym_custom(key,pubkey,
                AsymmetricSignatureAlgorithm::RsaPKCS15Sha256,
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1,
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256),

            SecurityPolicyUri::Basic128Rsa15=>self.set_asym_custom(key,pubkey,
                AsymmetricSignatureAlgorithm::RsaPKCS15Sha1,
            AsymmetricEncryptionAlgorithm::RsaPKCS15,
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha1),
            _=> {},
        }
    }

    fn set_asym_custom(&mut self,
        key: &'a PrivateKey,
        pubkey: &'a PublicKey,
        asym_sign:AsymmetricSignatureAlgorithm,
        asym_crypt:AsymmetricEncryptionAlgorithm,
        cert_sign:AsymmetricSignatureAlgorithm,
    ) {
        
        self.asymmetric_signature_algorithm=Some(AsymmetricSigner::new(
            key,
            pubkey,
            asym_sign,
        ));
        self.asymmetric_encryption= Some(EncrypterDecrypter::new(
            asym_crypt,
            key,
            pubkey,
        ));
        self.certificate_signature_algorithm= Some(AsymmetricSigner::new(
            key,
            pubkey,
            cert_sign,
        ));
    }

   

    pub fn update_symmetric_server(
        &mut self,
        symmetric_encryption_key_server: Vec<u8>,
        symmetric_signing_key_server: Vec<u8>,
        iv_server: Option<Vec<u8>>,
    ) {
        self.symmetric_signature_algorithm_server = Some(SymmetricSigner::new(
            symmetric_signing_key_server,
            self.symmetric_signature_algo.as_ref().unwrap().clone(),
        ));
        self.symmetric_encryption_server = Some(SymCipher::new(
            self.symmetric_encryption_algo.as_ref().unwrap().clone(),
            symmetric_encryption_key_server,
            iv_server,
        ));
    }
    pub fn update_symmetric_client(
        &mut self,
        symmetric_encryption_key_client: Vec<u8>,
        symmetric_signing_key_client: Vec<u8>,
        iv_client: Option<Vec<u8>>,
    ) {
        self.symmetric_signature_algorithm_client = Some(SymmetricSigner::new(
            symmetric_signing_key_client,
            self.symmetric_signature_algo.as_ref().unwrap().clone(),
        ));
        self.symmetric_encryption_client = Some(SymCipher::new(
            self.symmetric_encryption_algo.as_ref().unwrap().clone(),
            symmetric_encryption_key_client,
            iv_client,
        ));
    }
    //Only Symmetric signature/ecnryption algorithm  and key size depend on policy
    pub fn derive_symmetric_client  (
        &mut self,
        client_nonce: &ByteString,
        server_nonce: &ByteString,
    )-> MapperResult<()> {
        //verify that all required element are present
        let symmetric_encryption_algo=match self.symmetric_encryption_algo.as_ref(){
            Some(sym_encryption_algo)=>sym_encryption_algo,
            _ => return Err(MapperError::new(MapperErrorKind::KeyDerivation,"deserialize count failed 1"))
        };
        let symmetric_signature_algo=match self.symmetric_signature_algo.as_ref(){
            Some(sym_encryption_algo)=>sym_encryption_algo,
            _ => return Err(MapperError::new(MapperErrorKind::KeyDerivation,"deserialize count failed 2"))
        };
        let client_nonce=match client_nonce.value.as_deref(){
            Some(nonce)=>nonce,
            _ => return Err(MapperError::new(MapperErrorKind::KeyDerivation,"deserialize count failed 3"))
        };
        let server_nonce=match server_nonce.value.as_deref(){
            Some(nonce)=>nonce,
            _ => return Err(MapperError::new(MapperErrorKind::KeyDerivation,"deserialize count failed 4"))
        };
        let key_derivation=match &self.key_derivation_algorithm{
            Some(f)=>f,
            _ => return Err(MapperError::new(MapperErrorKind::KeyDerivation,"deserialize count failed 5"))
        };
        //offset to split the derivated vector into keys
        let total_length=self.initialization_vector_length+self.derived_signature_key_length+symmetric_encryption_algo.get_key_length();
        let iv_offset=total_length-self.initialization_vector_length;
        //derive server keys and create symmetric part of security policy
        let server_keys = key_derivation.derive_key(
            client_nonce,
            server_nonce,
            total_length,
        );
        self.symmetric_signature_algorithm_server = Some(SymmetricSigner::new(
            server_keys[..self.derived_signature_key_length].to_vec(),
            symmetric_signature_algo.clone(),
        ));
        self.symmetric_encryption_server = Some(SymCipher::new(
            symmetric_encryption_algo.clone(),
            server_keys[self.derived_signature_key_length..iv_offset].to_vec(),
            Some(server_keys[iv_offset..].to_vec()),
        ));

        //derive client keys and create symmetric part of security policy
        let client_keys = key_derivation.derive_key(
            server_nonce,
            client_nonce,
            total_length,
        );
        self.symmetric_signature_algorithm_client = Some(SymmetricSigner::new(
            client_keys[..self.derived_signature_key_length].to_vec(),
            symmetric_signature_algo.clone(),
        ));
        self.symmetric_encryption_client = Some(SymCipher::new(
            symmetric_encryption_algo.clone(),
            client_keys[self.derived_signature_key_length..iv_offset].to_vec(),
            Some(client_keys[iv_offset..].to_vec()),
        ));
        Ok(())
    }

    pub fn update_asymmetric(&mut self, key: &'a PrivateKey, pubkey: &'a PublicKey) {
        self.asymmetric_signature_algorithm = Some(AsymmetricSigner::new(
            key,
            pubkey,
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256,
        ));
        self.asymmetric_encryption = Some(EncrypterDecrypter::new(
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1,
            key,
            pubkey,
        ));
        self.certificate_signature_algorithm = Some(AsymmetricSigner::new(
            key,
            pubkey,
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256,
        ));
    }
}
