#[cfg(test)]
mod test {

    use crate::crypto::hash::*;
    use crate::crypto::tests::decode_hex;
    use openssl::x509::X509;
    use std::fs;
    #[test]
    fn certificate_thumbprint() {
        let der_cert = fs::read("src/crypto/tests/uaexpert.der").unwrap();
        let cert = X509::from_der(&der_cert).unwrap();
        let expected = decode_hex("6bfe3676b53f5edf26f76a606542ba00bfce54e4").unwrap();
        let result = compute_certificate_thumbprint(&cert);
        assert_eq!(expected, result);
    }
}
