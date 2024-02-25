use openssl::x509::X509;
///not required for the mapper now
pub struct TrustedList {
    pub(crate) trusted: Vec<X509>,
    pub(crate) ca: Option<X509>,
}
