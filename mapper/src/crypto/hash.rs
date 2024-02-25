use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;
use openssl::x509::X509;
/// Pseudo random `P_SHA` implementation for creating pseudo random range of bytes from an input
/// <https://www.ietf.org/rfc/rfc4346.txt>
/// <https://tools.ietf.org/html/rfc5246>
/// length is in bytes
pub fn p_hash(message_digest: MessageDigest, secret: &[u8], seed: &[u8], length: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(length);

    let mut hmac = Vec::with_capacity(seed.len() * 2);

    let mut a_last = Vec::with_capacity(seed.len());
    a_last.extend_from_slice(seed); // A(0) = seed

    while result.len() < length {
        let a_next = hmac_vec(message_digest, secret, &a_last);
        let bytes = {
            hmac.clear();
            hmac.extend_from_slice(&a_next);
            hmac.extend_from_slice(seed);
            hmac_vec(message_digest, secret, &hmac)
        };

        result.extend_from_slice(&bytes);
        a_last.clear();
        a_last.extend(a_next);
    }
    result[..length].to_vec()
}

/// hkdf to derive key
pub fn hkdf(message_digest: MessageDigest, salt: &[u8], ikm: &[u8], length: usize) -> Vec<u8> {
    let prk = hkdf_extract(&message_digest, salt, ikm);
    //in our case info is client salt for client or server salt for server ie the same as salt parameter
    hkdf_expand(&message_digest, &prk, salt, length)
}

pub(crate) fn hkdf_extract(message_digest: &MessageDigest, salt: &[u8], ikm: &[u8]) -> Vec<u8> {
    hmac_vec(*message_digest, salt, ikm)
}

//same T as in rfc5869
//ATTENTION the n should not exceed 255. That should not be problem but !!!
pub(crate) fn hkdf_expand(
    message_digest: &MessageDigest,
    prk: &[u8],
    info: &[u8],
    length: usize,
) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(length);
    let mut last_t: Vec<u8> = Vec::with_capacity(message_digest.size() + info.len() + 1);
    let n: usize = length / message_digest.size() + 1;
    for it in 1..n + 1 {
        let t = {
            last_t.extend(info);
            last_t.extend_from_slice(&(it as u8).to_le_bytes());
            hmac_vec(*message_digest, prk, &last_t)
        };
        result.extend_from_slice(&t);
        last_t.clear();
        last_t.extend(t);
    }
    result[..length].to_vec()
}

pub fn hmac_vec(digest: MessageDigest, key: &[u8], data: &[u8]) -> Vec<u8> {
    // Compute a hmac
    let pkey = PKey::hmac(key).unwrap();
    let mut signer = Signer::new(digest, &pkey).unwrap();
    signer.update(data).unwrap();
    signer.sign_to_vec().unwrap()
}

/// This function compute the thumbprint of a certificate. It is a wrapper
/// to the underlying function in openssl library. OPC-UA requires sha1 to be used.
pub fn compute_certificate_thumbprint(cert: &X509) -> Vec<u8> {
    cert.digest(MessageDigest::sha1()).unwrap().to_vec()
}

pub fn compute_certificate_thumbprint_from_file(cert_path: &str) -> Vec<u8> {
    let certificate = X509::from_der(&std::fs::read(cert_path).unwrap()).unwrap();
    compute_certificate_thumbprint(&certificate)
}
