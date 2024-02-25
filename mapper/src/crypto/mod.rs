//! This module is responsible for cryptographic operation and material.
//! Nevertheless it works on vector and not on message. To use it on message
//! please use the look into the msg part.

pub mod encryption;
pub mod hash;
pub mod pkey;
pub mod random;
pub mod security_policy;
pub mod signature;
pub mod x509;
pub mod raw_rsa;

pub mod encryption_prelude{
    pub use super::encryption::EncrypterDecrypter;
    pub use super::encryption::SymCipher;
    pub use super::encryption::AsymmetricEncryptionAlgorithm;
    pub use super::encryption::SymmetricEncryptionAlgorithm;
}

pub mod signature_prelude{
    pub use super::signature::AsymmetricSigner;
    pub use super::signature::SymmetricSigner;
    pub use super::signature::AsymmetricSignatureAlgorithm;
    pub use super::signature::SymmetricSignatureAlgorithm;
}

pub mod policy_prelude{
    pub use super::security_policy::SecurityPolicy;
    pub use super::security_policy::SecurityPolicyUri;
}
#[cfg(test)]
mod tests;
