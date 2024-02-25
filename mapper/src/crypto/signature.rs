use crate::crypto::encryption::RsaPadding;

use super::pkey::{PrivateKey, PublicKey};
use crypto::mac::Mac;
use crypto::poly1305::Poly1305;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::{self, RsaPssSaltlen};
// use openssl::pkey::PKey;

///general signer struct that can either use symmetric or asymmetric but not both
pub enum Signer<'a> {
    SymmetricSigner(SymmetricSigner),
    AsymmetricSigner(AsymmetricSigner<'a>),
    None,
}

#[derive(Clone)]
///List of supported symmetric signature algorithm
pub enum SymmetricSignatureAlgorithm {
    HmacSha1,
    HmacSha256,
    HmacSha384,
    Poly1305,
}


#[derive(Debug)]
///List of supported asymmetric signature algorithm
pub enum AsymmetricSignatureAlgorithm {
    EcdsaSha256,
    EcdsaSha384,
    RsaPKCS15Sha1,
    RsaPKCS15Sha256,
    RsaPssSha256,
    PureEdDSA25519,
    PureEdDSA448,
}
//pour les certificats il faut bien regarder les détails de la doc. En effet "The SHA2 algorithm with 384 or 512 bits may be used instead of SHA2 with 256 bits. " puor
//CertificateSignatureAlgorithm_RSA-PKCS15-SHA2-256
///struct that handle symmetric signature
#[derive(Clone)]
pub struct SymmetricSigner {
    key: Vec<u8>,
    symmetric_signature_algorithm: SymmetricSignatureAlgorithm,
}

impl SymmetricSigner {
    //create a new signer
    pub fn new(
        key: Vec<u8>,
        symmetric_signature_algorithm: SymmetricSignatureAlgorithm,
    ) -> SymmetricSigner {
        SymmetricSigner {
            key,
            symmetric_signature_algorithm,
        }
    }
    ///sign the data according the algorithm of the struct.
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let key = PKey::hmac(&self.key).unwrap();
        match self.symmetric_signature_algorithm {
            SymmetricSignatureAlgorithm::HmacSha1 => {
                let mut signer = sign::Signer::new(MessageDigest::sha1(), &key).unwrap();
                signer.update(data).unwrap();
                signer.sign_to_vec().unwrap()
            }
            SymmetricSignatureAlgorithm::HmacSha256 => {
                let mut signer = sign::Signer::new(MessageDigest::sha256(), &key).unwrap();
                signer.update(data).unwrap();
                signer.sign_to_vec().unwrap()
            }
            SymmetricSignatureAlgorithm::HmacSha384 => {
                let mut signer = sign::Signer::new(MessageDigest::sha384(), &key).unwrap();
                signer.update(data).unwrap();
                signer.sign_to_vec().unwrap()
            }
            SymmetricSignatureAlgorithm::Poly1305 => {
                let mut poly = Poly1305::new(&self.key);
                poly.input(data);
                poly.result().code().to_vec()
            }
        }
    }

    ///verify the signature with the algortihme in the signer
    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        let key = PKey::hmac(&self.key).unwrap();
        match self.symmetric_signature_algorithm {
            SymmetricSignatureAlgorithm::HmacSha1 => {
                let mut signer = sign::Signer::new(MessageDigest::sha1(), &key).unwrap();
                signer.update(data).unwrap();
                let hmac = signer.sign_to_vec().unwrap();
                openssl::memcmp::eq(&hmac, &signature)
            }
            SymmetricSignatureAlgorithm::HmacSha256 => {
                let mut signer = sign::Signer::new(MessageDigest::sha256(), &key).unwrap();
                signer.update(data).unwrap();
                let hmac = signer.sign_to_vec().unwrap();
                openssl::memcmp::eq(&hmac, &signature)
            }
            SymmetricSignatureAlgorithm::HmacSha384 => {
                let mut signer = sign::Signer::new(MessageDigest::sha384(), &key).unwrap();
                signer.update(data).unwrap();
                let hmac = signer.sign_to_vec().unwrap();
                openssl::memcmp::eq(&hmac, &signature)
            }
            SymmetricSignatureAlgorithm::Poly1305 => {
                let mut poly = Poly1305::new(&self.key);
                poly.input(data);
                let hmac = poly.result().code().to_vec();
                openssl::memcmp::eq(&hmac, &signature)
            }
        }
    }
    ///hardcoded value of signature size. It is usefull since Poly1305 use a different API.
    pub fn signature_size(&self) -> usize {
        match self.symmetric_signature_algorithm {
            SymmetricSignatureAlgorithm::HmacSha1 => 20,
            SymmetricSignatureAlgorithm::HmacSha256 => 32,
            SymmetricSignatureAlgorithm::HmacSha384 => 48,
            SymmetricSignatureAlgorithm::Poly1305 => 20,
        }
    }
}

///struct that handle asymmetric signature
///The lifetime is required because the underlying openssl struct
/// requires key to be references.

pub struct AsymmetricSigner<'a> {
    pub(crate) key: &'a PrivateKey,
    pub(crate) pubkey: &'a PublicKey,
    pub(crate) signer: sign::Signer<'a>,
    pub(crate) verifier: sign::Verifier<'a>,
    pub(crate) asymmetric_signature_algorithm: AsymmetricSignatureAlgorithm,
}

impl<'a> AsymmetricSigner<'a> {
    pub fn new(
        key: &'a PrivateKey,
        pubkey: &'a PublicKey,
        asymmetric_signature_algorithm: AsymmetricSignatureAlgorithm,
    ) -> AsymmetricSigner<'a> {
        AsymmetricSigner {
            key,
            pubkey,
            signer: AsymmetricSigner::get_signer_from_algorithm(
                key,
                &asymmetric_signature_algorithm,
            ),
            verifier: AsymmetricSigner::get_verifier_from_algorithm(
                pubkey,
                &asymmetric_signature_algorithm,
            ),
            asymmetric_signature_algorithm,
        }
    }

    //Do not try to get the result from pss_saltlen, error of the library is a false error.
    fn get_signer_from_algorithm(
        key: &'a PrivateKey,
        asymmetric_signature_algorithm: &AsymmetricSignatureAlgorithm,
    ) -> sign::Signer<'a> {
        match *asymmetric_signature_algorithm {
            AsymmetricSignatureAlgorithm::EcdsaSha256 => {
                let signer = sign::Signer::new(MessageDigest::sha1(), key).unwrap();
                signer
            }
            AsymmetricSignatureAlgorithm::EcdsaSha384 => {
                let signer = sign::Signer::new(MessageDigest::sha384(), key).unwrap();
                signer
            }
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha1 => {
                let mut signer =
                    sign::Signer::new(openssl::hash::MessageDigest::sha1(), key).unwrap();
                signer.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                let _ = signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                signer
            }
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256 => {
                let mut signer = sign::Signer::new(MessageDigest::sha256(), key).unwrap();
                signer.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                let _ = signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                signer
            }
            AsymmetricSignatureAlgorithm::RsaPssSha256 => {
                let mut signer = sign::Signer::new(MessageDigest::sha256(), key).unwrap();
                signer.set_rsa_padding(RsaPadding::Pkcs1Pss.into()).unwrap();
                let _ = signer.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                signer
            }
            AsymmetricSignatureAlgorithm::PureEdDSA25519 => {
                let signer = sign::Signer::new_without_digest(key).unwrap();
                signer
            }
            AsymmetricSignatureAlgorithm::PureEdDSA448 => {
                let signer = sign::Signer::new_without_digest(key).unwrap();
                signer
            }
        }
    }

    fn get_verifier_from_algorithm(
        key: &'a PublicKey,
        asymmetric_signature_algorithm: &AsymmetricSignatureAlgorithm,
    ) -> sign::Verifier<'a> {
        match *asymmetric_signature_algorithm {
            AsymmetricSignatureAlgorithm::EcdsaSha256 => {
                let verifier = sign::Verifier::new(MessageDigest::sha1(), key).unwrap();
                verifier
            }
            AsymmetricSignatureAlgorithm::EcdsaSha384 => {
                let verifier = sign::Verifier::new(MessageDigest::sha384(), key).unwrap();
                verifier
            }
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha1 => {
                let mut verifier = sign::Verifier::new(MessageDigest::sha1(), key).unwrap();
                verifier.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                let _ = verifier.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                verifier
            }
            AsymmetricSignatureAlgorithm::RsaPKCS15Sha256 => {
                let mut verifier = sign::Verifier::new(MessageDigest::sha256(), key).unwrap();
                verifier.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                let _ = verifier.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                verifier
            }
            AsymmetricSignatureAlgorithm::RsaPssSha256 => {
                let mut verifier = sign::Verifier::new(MessageDigest::sha256(), key).unwrap();
                verifier
                    .set_rsa_padding(RsaPadding::Pkcs1Pss.into())
                    .unwrap();
                let _ = verifier.set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH);
                verifier
            }
            AsymmetricSignatureAlgorithm::PureEdDSA25519 => {
                let verifier = sign::Verifier::new_without_digest(key).unwrap();
                verifier
            }
            AsymmetricSignatureAlgorithm::PureEdDSA448 => {
                let verifier = sign::Verifier::new_without_digest(key).unwrap();
                verifier
            }
        }
    }
    /// Sign the data. the rust wrapper keep using the same MD_CTX so we need a new signer each time.
    pub fn sign(&mut self, data: &[u8]) -> Vec<u8> {
        let result=self.signer.sign_oneshot_to_vec(data).unwrap();
        self.signer=AsymmetricSigner::get_signer_from_algorithm(self.key,&self.asymmetric_signature_algorithm);
        result
    }
    /// Verify the signature of the data
    pub fn verify(&mut self, data: &[u8], signature: &[u8]) -> bool {
        let res=self.verifier.verify_oneshot(signature, data).unwrap();
        self.verifier=AsymmetricSigner::get_verifier_from_algorithm(self.pubkey,&self.asymmetric_signature_algorithm);
        res
    }
    /// return the signature size
    pub fn signature_size(&self) -> usize {
        self.key.size()
    }
}
