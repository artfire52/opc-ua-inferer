use crate::{Deserialize, Serialize, MapperResult};

//Not usefull for now
pub (crate)  enum Footer {
    AutenticatedEncryption(AuthenticatedEncryptionFooter),
    UnAuthenticatedEncryption(UnAuthenticatedEncryptionFooter),
}

pub (crate)  struct AuthenticatedEncryptionFooter {
    pub(crate) signature: Vec<u8>,
}

pub (crate)  struct UnAuthenticatedEncryptionFooter {
    pub(crate) padding_size: u8,
    pub(crate) padding: Vec<u8>,
    pub(crate) extra_padding_size: u8,
    pub(crate) signature: Vec<u8>,
    pub(crate) is_key_less_than_2048bits: bool,
}

impl Serialize for AuthenticatedEncryptionFooter {
    fn serialize(&self) -> Vec<u8> {
        self.signature.clone()
    }
}
impl AuthenticatedEncryptionFooter {
    pub fn add_footer(&self, packet: &mut Vec<u8>) {
        packet.extend_from_slice(&self.signature)
    }

    pub fn new(signature: Vec<u8>) -> AuthenticatedEncryptionFooter {
        AuthenticatedEncryptionFooter {
            signature: signature,
        }
    }
}
impl Serialize for UnAuthenticatedEncryptionFooter {
    fn serialize(&self) -> Vec<u8> {
        let mut result = vec![self.padding_size];
        result.extend_from_slice(&self.padding);
        if !self.is_key_less_than_2048bits {
            result.extend_from_slice(&self.extra_padding_size.serialize());
        }
        result.extend_from_slice(&self.signature);
        result
    }
}

impl UnAuthenticatedEncryptionFooter {
    fn add_footer(&self, packet: &mut Vec<u8>) {
        packet.extend_from_slice(&self.serialize())
    }
}

impl Deserialize for AuthenticatedEncryptionFooter {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        Ok((
            &[],
            AuthenticatedEncryptionFooter::new(data.to_vec().clone()),
        ))
    }
}

impl Deserialize for UnAuthenticatedEncryptionFooter {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, padding_size) = u8::deserialize(data)?;
        let (signature, padding) =
            UnAuthenticatedEncryptionFooter::take_count(data, padding_size as usize)?;
        let signature = signature.to_vec();
        // let (signature,extra_padding)= UnAuthenticatedEncryptionFooter::take_count(&data,1).unwrap();
        // let padding_size=extra_padding[0];
        // let signature=signature.to_vec();
        Ok((
            &[],
            UnAuthenticatedEncryptionFooter {
                padding_size: padding_size,
                padding: padding,
                extra_padding_size: 0,
                signature: signature,
                is_key_less_than_2048bits: true,
            },
        ))
    }
}
