#[cfg(test)]
mod test {
    use crate::crypto::hash;
    use crate::crypto::{hash::p_hash, random, tests::decode_hex};
    use openssl::hash::MessageDigest;

    #[test]
    fn psha_256_derive_size_test() {
        assert!(true);
        let secret = random::byte_string(32);
        let seed = random::byte_string(32);
        let generated_random = p_hash(
            MessageDigest::sha256(),
            &secret.value.unwrap(),
            &seed.value.unwrap(),
            3052,
        );
        assert!(generated_random.len() == 3052);
    }

    #[test]
    fn hkdf_sha256_rfc5869_test_case_2() {
        let ikm  =decode_hex("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f").unwrap(); // (80 octets)
        let salt = decode_hex("606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeaf").unwrap(); // (80 octets)
        let info = decode_hex(
            "b0b1b2b3b4b5b6b7b8b9babbbcbdbebf\
                                c0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
                                d0d1d2d3d4d5d6d7d8d9dadbdcdddedf\
                                e0e1e2e3e4e5e6e7e8e9eaebecedeeef\
                                f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff",
        )
        .unwrap(); // (80 octets)
        let l = 82;

        let prk =
            decode_hex("06a6b88c5853361a06104c9ceb35b45cef760014904671014a193f40c15fc244").unwrap(); // (32 octets)
        let okm = decode_hex(
            "b11e398dc80327a1c8e7f78c596a4934\
                                4f012eda2d4efad8a050cc4c19afa97c\
                                59045a99cac7827271cb41c65e590e09\
                                da3275600c2f09b8367793a9aca3db71\
                                cc30c58179ec3e87c14c01d5c1f3434f\
                                1d87",
        )
        .unwrap(); // 82 octets

        // hash::hkdf(MessageDigest::sha256(), &salt, &ikm, 82);
        let computed_prk = hash::hkdf_extract(&MessageDigest::sha256(), &salt, &ikm);
        assert_eq!(prk, computed_prk);
        let res = hash::hkdf_expand(&MessageDigest::sha256(), &prk, &info, l);
        assert_eq!(res, okm);
    }

    #[test]
    fn test_psha_256() {
        //key size : 32 32 16
        let client_nonce =
            decode_hex("A0D7AF32BFF200C60C672C3F11EB844F7FEB0CB4D41C3DD5FCB3125899D024EB").unwrap();
        let server_nonce =
            decode_hex("3B86733EF12CE3A142BB61B1E7435E2966E7AAE24AF0615F5C10B44451376EF5").unwrap();
        let server_keys_expected=decode_hex("104DAC07DC239EACF05303E5CA4D98B2DB7059F67EAD2B284AD1FC2518745F566ED3340D5BDC6A0A270D74A14CCDA12A6204F288D64BBA78C1F487012C085B456D1F4912BB80BF791CCCB53297ABEDAE").unwrap();
        let client_keys_expected=decode_hex("31474E253B30363740CA8F015913A3075CF9E435D0731523B78E61ED58CB468E6EB3D902D6BD712B3E9CA7FB016EB29010893998EC29561EB605DDC39B13505EA217A7ECC911528AE61F069EE680BCA7").unwrap();
        let computed_client_key = hash::p_hash(
            openssl::hash::MessageDigest::sha256(),
            &server_nonce,
            &client_nonce,
            32 + 32 + 16,
        );
        let computed_server_key = hash::p_hash(
            openssl::hash::MessageDigest::sha256(),
            &client_nonce,
            &server_nonce,
            32 + 32 + 16,
        );
        assert_eq!(computed_client_key, client_keys_expected);
        assert_eq!(computed_server_key, server_keys_expected);
    }
}
