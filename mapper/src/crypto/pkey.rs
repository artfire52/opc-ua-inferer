use openssl::pkey::{PKey, Private, Public};

/// A public key
pub type PublicKey = PKey<Public>;
/// A private key
pub type PrivateKey = PKey<Private>;
