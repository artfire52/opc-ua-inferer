use super::string::UaString;
use crate::encoding_prelude::*;

#[derive(Debug, PartialEq,Clone)]
pub struct LocalizedText {
    pub(crate) encoding_mask: u8,
    pub(crate) locale: UaString,
    pub(crate) text: UaString,
}

pub struct EncodingValue;
impl EncodingValue {
    pub const LOCALE: u8 = 0x01;
    pub const TEXT: u8 = 0x02;
}

impl Serialize for LocalizedText {
    fn serialize(&self) -> Vec<u8> {
        let mut result = vec![0x00];
        if !self.locale.isnull() {
            result[0] = result[0] | EncodingValue::LOCALE;
            result.extend_from_slice(&mut self.locale.serialize());
        }
        if !self.text.isnull() {
            result[0] = result[0] | EncodingValue::TEXT;
            result.extend_from_slice(&mut self.text.serialize());
        }
        result
    }
}

impl Deserialize for LocalizedText {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (mut data, encoding) = u8::deserialize(data)?;
        let mut locale = UaString::new();
        let mut text = UaString::new();
        if encoding & EncodingValue::LOCALE == EncodingValue::LOCALE {
            (data, locale) = UaString::deserialize(data)?;
        }
        if encoding & EncodingValue::TEXT == EncodingValue::TEXT {
            (data, text) = UaString::deserialize(data)?;
        }
        Ok((
            data,
            LocalizedText {
                encoding_mask: encoding,
                locale,
                text,
            },
        ))
    }
}

impl LocalizedText {
    pub fn new(locale: UaString, text: UaString) -> LocalizedText {
        LocalizedText {
            encoding_mask: 0,
            locale,
            text,
        }
    }
    pub fn new_empty() -> LocalizedText {
        LocalizedText {
            encoding_mask: 0,
            locale: UaString::new(),
            text: UaString::new(),
        }
    }
}
