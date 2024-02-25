use openssl::encrypt::{Decrypter, Encrypter};
use openssl::hash::MessageDigest;
use openssl::rsa;
use openssl::symm;

use super::pkey::{PrivateKey, PublicKey};

///Enumeration of supported encryption algortihm (asymmetric or symmetric).
#[derive(Copy, Clone)]
pub enum EncryptionAlgorithm {
    Symmetric(SymmetricEncryptionAlgorithm),
    Asymmetric(AsymmetricEncryptionAlgorithm),
    None,
}
pub enum EncryptionType {
    Symmetric,
    Asymmetric,
    None,
}

///Enumeration of supported asymmetric encryption algortihm.
#[derive(Copy, Clone)]
pub enum SymmetricEncryptionAlgorithm {
    Aes128CBC,
    Aes256CBC,
    Aes128CTR,
    Aes256CTR,
    ChaCha20Poly1305,
}

impl SymmetricEncryptionAlgorithm {
    pub fn get_cipher(&self) -> symm::Cipher {
        match self {
            SymmetricEncryptionAlgorithm::Aes128CBC => symm::Cipher::aes_128_cbc(),
            SymmetricEncryptionAlgorithm::Aes256CBC => symm::Cipher::aes_256_cbc(),
            SymmetricEncryptionAlgorithm::Aes128CTR => symm::Cipher::aes_128_ctr(),
            SymmetricEncryptionAlgorithm::Aes256CTR => symm::Cipher::aes_256_ctr(),
            SymmetricEncryptionAlgorithm::ChaCha20Poly1305 => symm::Cipher::chacha20_poly1305(),
        }
    }
    pub fn get_key_length(&self) -> usize {
        self.get_cipher().key_len()
    }
}

///Enumeration of supported asymmetric encryption algortihm.
#[derive(Copy, Clone)]
pub enum AsymmetricEncryptionAlgorithm {
    RsaPKCS15,
    RsaOAEPSHA1,
    RsaOAEPSHA2256,
}

#[derive(Debug, PartialEq)]
pub(crate) enum RsaPadding {
    Pkcs1,
    OaepSha1,
    OaepSha256,
    Pkcs1Pss,
}

impl Into<rsa::Padding> for RsaPadding {
    fn into(self) -> rsa::Padding {
        match self {
            RsaPadding::Pkcs1 => rsa::Padding::PKCS1,
            RsaPadding::OaepSha1 => rsa::Padding::PKCS1_OAEP,
            RsaPadding::Pkcs1Pss => rsa::Padding::PKCS1_PSS,
            // Note: This is the right padding but not the right hash and must be handled by special case in the code
            RsaPadding::OaepSha256 => rsa::Padding::PKCS1_OAEP,
        }
    }
}

///Struct that handle encryption and decryption for asymmetric operation.

pub struct EncrypterDecrypter<'a> {
    pub(crate) encrypter: Encrypter<'a>,
    pub(crate) decrypter: Decrypter<'a>,
    pub(crate) padding: RsaPadding,
}

impl<'a> EncrypterDecrypter<'a> {
    pub fn new(
        algo: AsymmetricEncryptionAlgorithm,
        key: &'a PrivateKey,
        pubkey: &'a PublicKey,
    ) -> EncrypterDecrypter<'a> {
        let mut encrypter = Encrypter::new(&pubkey).unwrap();
        let mut decrypter = Decrypter::new(&key).unwrap();
        match algo {
            AsymmetricEncryptionAlgorithm::RsaPKCS15 => {
                encrypter.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                decrypter.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
                EncrypterDecrypter {
                    encrypter: encrypter,
                    decrypter: decrypter,
                    padding: RsaPadding::Pkcs1,
                }
            }
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1 => {
                encrypter
                    .set_rsa_padding(RsaPadding::OaepSha1.into())
                    .unwrap();
                decrypter
                    .set_rsa_padding(RsaPadding::OaepSha1.into())
                    .unwrap();
                let _ = encrypter.set_rsa_oaep_md(MessageDigest::sha1());
                let _ = decrypter.set_rsa_oaep_md(MessageDigest::sha1());
                EncrypterDecrypter {
                    encrypter: encrypter,
                    decrypter: decrypter,
                    padding: RsaPadding::OaepSha1,
                }
            }

            AsymmetricEncryptionAlgorithm::RsaOAEPSHA2256 => {
                encrypter
                    .set_rsa_padding(RsaPadding::OaepSha256.into())
                    .unwrap();
                decrypter
                    .set_rsa_padding(RsaPadding::OaepSha256.into())
                    .unwrap();
                let _ = encrypter.set_rsa_oaep_md(MessageDigest::sha256());
                let _ = decrypter.set_rsa_oaep_md(MessageDigest::sha256());
                EncrypterDecrypter {
                    encrypter: encrypter,
                    decrypter: decrypter,
                    padding: RsaPadding::OaepSha256,
                }
            }
        }
    }
    
    pub(crate) fn set_padding_encrypter(encrypter:&mut Encrypter,algo: AsymmetricEncryptionAlgorithm){
        match algo {
            AsymmetricEncryptionAlgorithm::RsaPKCS15 => {
                encrypter.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
            }
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1 => {
                encrypter
                    .set_rsa_padding(RsaPadding::OaepSha1.into())
                    .unwrap();
                let _ =   encrypter.set_rsa_oaep_md(MessageDigest::sha1());
            }

            AsymmetricEncryptionAlgorithm::RsaOAEPSHA2256 => {
                encrypter
                    .set_rsa_padding(RsaPadding::OaepSha256.into())
                    .unwrap();
                let _ = encrypter.set_rsa_oaep_md(MessageDigest::sha256());
            }
        }

    }
    pub(crate) fn set_padding_decrypter(decrypter:&mut Decrypter,algo: AsymmetricEncryptionAlgorithm){
        match algo {
            AsymmetricEncryptionAlgorithm::RsaPKCS15 => {
                decrypter.set_rsa_padding(RsaPadding::Pkcs1.into()).unwrap();
            }
            AsymmetricEncryptionAlgorithm::RsaOAEPSHA1 => {
                decrypter
                    .set_rsa_padding(RsaPadding::OaepSha1.into())
                    .unwrap();
                let _ =   decrypter.set_rsa_oaep_md(MessageDigest::sha1());
            }

            AsymmetricEncryptionAlgorithm::RsaOAEPSHA2256 => {
                decrypter
                    .set_rsa_padding(RsaPadding::OaepSha256.into())
                    .unwrap();
                let _ = decrypter.set_rsa_oaep_md(MessageDigest::sha256());
            }
        }

    }
    pub(crate) fn plain_text_block_size(&self, src: &[u8]) -> usize {
        // flen must not be more than RSA_size(rsa) - 11 for the PKCS #1 v1.5 based padding modes,
        // not more than RSA_size(rsa) - 42 for RSA_PKCS1_OAEP_PADDING and exactly RSA_size(rsa)
        // for RSA_NO_PADDING.
        let size = self.encrypter.encrypt_len(src).unwrap();
        match self.padding {
            RsaPadding::Pkcs1 => size - 11,
            RsaPadding::OaepSha1 => size - 42,
            RsaPadding::OaepSha256 => size - 66,
            _ => panic!("Unsupported padding"),
        }
    }

    pub fn encrypt(&self, src: &[u8]) -> Vec<u8> {
        let encrypter = &self.encrypter;
        let cipher_text_block_size = self.encrypter.encrypt_len(src).unwrap();
        let plain_text_block_size = self.plain_text_block_size(src);
        let mut dst = vec![];

        // Encrypt the data in chunks no larger than the key size less padding
        let mut src_idx = 0;
        let mut dst_idx = 0;

        let src_len = src.len();
        while src_idx < src_len {
            dst.resize(dst.len() + cipher_text_block_size, 0);
            let bytes_to_encrypt = if src_len < plain_text_block_size {
                src_len
            } else if (src_len - src_idx) < plain_text_block_size {
                src_len - src_idx
            } else {
                plain_text_block_size
            };

            // Encrypt data, advance dst index by number of bytes after encrypted
            dst_idx += {
                let src = &src[src_idx..(src_idx + bytes_to_encrypt)];
                let dst = &mut dst[dst_idx..(dst_idx + cipher_text_block_size)];

                encrypter.encrypt(src, dst).unwrap()
            };

            // Src advances by bytes to encrypt
            src_idx += bytes_to_encrypt;
        }

        dst.truncate(dst_idx);
        dst
    }

    pub fn decrypt(&mut self, src: &[u8]) -> Vec<u8> {
        // decrypt data using our private key

        let decrypter = &self.decrypter;
        let cipher_text_block_size = decrypter.decrypt_len(src).unwrap();
        let mut dst = vec![];

        // Decrypt the data
        let mut src_idx = 0;
        let mut dst_idx = 0;
        let src_len = src.len();
        while src_idx < src_len {
            // Decrypt and advance
            dst.resize(dst.len() + cipher_text_block_size, 0);
            dst_idx += {
                let src = &src[src_idx..(src_idx + cipher_text_block_size)];
                let dst = &mut dst[dst_idx..(dst_idx + cipher_text_block_size)];

                decrypter.decrypt(src, dst).unwrap()
            };
            src_idx += cipher_text_block_size;
        }
        let mut res = dst.to_vec();
        res.truncate(dst_idx);
        res
    }
}

///Struct that handle symmetric encryption and decryption.
#[derive(Clone)]

pub struct SymCipher {
    pub(crate) symmetric_encryption_algorithm: SymmetricEncryptionAlgorithm,
    pub(crate) key: Vec<u8>,
    pub(crate) iv: Option<Vec<u8>>,
}

impl SymCipher {
    pub fn new(t: SymmetricEncryptionAlgorithm, key: Vec<u8>, iv: Option<Vec<u8>>) -> SymCipher {
        SymCipher {
            symmetric_encryption_algorithm: t,
            key,
            iv,
        }
    }
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // let iv=self.iv.as_deref();
        // symm::encrypt(self.symmetric_encryption_algorithm.get_cipher(),&self.key,iv,data).unwrap()
        let mut encrypter = openssl::symm::Crypter::new(
            self.symmetric_encryption_algorithm.get_cipher(),
            symm::Mode::Encrypt,
            &self.key,
            self.iv.as_deref(),
        )
        .unwrap();
        encrypter.pad(false);
        let block_size = self
            .symmetric_encryption_algorithm
            .get_cipher()
            .block_size();
        let mut plaintext = vec![0; data.len() + block_size];
        let mut count = encrypter.update(&data, &mut plaintext).unwrap();
        count += encrypter.finalize(&mut plaintext[count..]).unwrap();
        plaintext.truncate(count);
        plaintext
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // let iv=self.iv.as_deref();
        // symm::decrypt(self.symmetric_encryption_algorithm.get_cipher(),&self.key,iv,data).unwrap()
        let mut decrypter = openssl::symm::Crypter::new(
            self.symmetric_encryption_algorithm.get_cipher(),
            symm::Mode::Decrypt,
            &self.key,
            self.iv.as_deref(),
        )
        .unwrap();
        decrypter.pad(false);
        let block_size = self
            .symmetric_encryption_algorithm
            .get_cipher()
            .block_size();
        let mut plaintext = vec![0; data.len() + block_size];
        let mut count = decrypter.update(&data, &mut plaintext).unwrap();
        count += decrypter.finalize(&mut plaintext[count..]).unwrap();
        plaintext.truncate(count);
        plaintext
    }

    pub fn plain_text_block_size_key_size(&self) -> (usize, usize) {
        let cipher = self.symmetric_encryption_algorithm.get_cipher();
        (cipher.block_size(), cipher.key_len())
    }

    pub fn key_size(&self) -> usize {
        let cipher = self.symmetric_encryption_algorithm.get_cipher();
        cipher.key_len()
    }
}

/// Contain either an symmetric or an asymmetric encryption struct
pub enum EncryptionMethod<'a> {
    Symmetric(SymCipher),
    AsymmetricPrivateEncrypterDecrypter(EncrypterDecrypter<'a>),
    None,
}

impl<'a> EncryptionMethod<'a> {
    fn new_symmetric(
        algo: SymmetricEncryptionAlgorithm,
        key: Vec<u8>,
        iv: Option<Vec<u8>>,
    ) -> EncryptionMethod<'a> {
        EncryptionMethod::Symmetric(SymCipher::new(algo, key, iv))
    }

    pub fn new_asymmetric_encrypter_decrypter(
        algo: AsymmetricEncryptionAlgorithm,
        key: &'a PrivateKey,
        pubkey: &'a PublicKey,
    ) -> EncryptionMethod<'a> {
        EncryptionMethod::AsymmetricPrivateEncrypterDecrypter(EncrypterDecrypter::new(
            algo, key, pubkey,
        ))
    }

    pub fn none() -> EncryptionMethod<'a> {
        EncryptionMethod::None
    }

    pub fn symmetric_encryption(cipher: &SymCipher, data: &[u8]) -> Vec<u8> {
        cipher.encrypt(data)
    }

    pub fn symmetric_decryption(cipher: &SymCipher, data: &[u8]) -> Vec<u8> {
        cipher.decrypt(data)
    }

    pub fn asymmetric_encrytption(encrypter: &mut Encrypter, data: &[u8]) -> Vec<u8> {
        let result_len = encrypter.encrypt_len(&data).unwrap();
        let mut result = vec![0u8; result_len];
        let endecoded_size = encrypter.encrypt(&data, &mut result).unwrap();
        result.truncate(endecoded_size);
        result
    }

    pub fn asymmetric_decrytption(decrypter: &mut Decrypter, data: &[u8]) -> Vec<u8> {
        let result_len = decrypter.decrypt_len(&data).unwrap();
        let mut result = vec![0u8; result_len];
        let decoded_size = decrypter.decrypt(&data, &mut result).unwrap();
        result.truncate(decoded_size);
        result
    }
}

///This struct is able to handle asymmetric and Symmetric encryption operation.
/// It is a wrapper around one encryption object.
pub struct Encryption<'a> {
    encryption_algorithm: EncryptionAlgorithm,
    encryption_method: EncryptionMethod<'a>,
}

impl<'a> Encryption<'a> {
    fn new(
        encryption_algorithm: EncryptionAlgorithm,
        encryption_method: EncryptionMethod<'a>,
    ) -> Encryption<'a> {
        Encryption {
            encryption_algorithm,
            encryption_method,
        }
    }
    pub fn new_symmetric(
        algo: SymmetricEncryptionAlgorithm,
        key: Vec<u8>,
        iv: Option<Vec<u8>>,
    ) -> Encryption<'a> {
        let encryption_method = EncryptionMethod::Symmetric(SymCipher::new(algo, key, iv));
        Encryption {
            encryption_algorithm: EncryptionAlgorithm::Symmetric(algo),
            encryption_method,
        }
    }

    pub fn new_asymmetric_encrypter_decrypter(
        algo: AsymmetricEncryptionAlgorithm,
        key: &'a PrivateKey,
        pubkey: &'a PublicKey,
    ) -> Encryption<'a> {
        let encryption_method = EncryptionMethod::AsymmetricPrivateEncrypterDecrypter(
            EncrypterDecrypter::new(algo, key, pubkey),
        );
        Encryption {
            encryption_algorithm: EncryptionAlgorithm::Asymmetric(algo),
            encryption_method,
        }
    }

    pub fn none() -> Encryption<'a> {
        Encryption {
            encryption_algorithm: EncryptionAlgorithm::None,
            encryption_method: EncryptionMethod::none(),
        }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        match &mut self.encryption_method {
            EncryptionMethod::Symmetric(sym_cipher) => {
                EncryptionMethod::symmetric_encryption(sym_cipher, data)
            }
            EncryptionMethod::AsymmetricPrivateEncrypterDecrypter(encrypter_decrypter) => {
                EncryptionMethod::asymmetric_encrytption(&mut encrypter_decrypter.encrypter, data)
            }
            EncryptionMethod::None => data.to_vec(),
        }
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        match &mut self.encryption_method {
            EncryptionMethod::Symmetric(sym_cipher) => {
                EncryptionMethod::symmetric_decryption(sym_cipher, data)
            }
            EncryptionMethod::AsymmetricPrivateEncrypterDecrypter(encrypter_decrypter) => {
                EncryptionMethod::asymmetric_decrytption(&mut encrypter_decrypter.decrypter, data)
            }
            _ => data.to_vec(),
        }
    }
}
