use super::{status_code::StatusCode, string::UaString};
use crate::{Deserialize, Serialize, MapperResult};

#[derive(Debug, PartialEq,Clone)]
pub struct DiagnosticInfo {
    pub(crate) encoding_mask: u8,
    pub(crate) symbolic_id: i32,
    pub(crate) namespace_uri: i32,
    pub(crate) locale: i32,
    pub(crate) localized_text: i32,
    pub(crate) additional_info: UaString,
    pub(crate) inner_status_code: u32,
    pub(crate) inner_diagnostic_info: Option<Box<DiagnosticInfo>>,
}

pub struct EncodingValue;
impl EncodingValue {
    pub const SYMBOLIC_ID: u8 = 0x01;
    pub const NAMESPACE: u8 = 0x02;
    pub const LOCALIZED_TEXT: u8 = 0x04;
    pub const LOCALE: u8 = 0x08;
    pub const ADDITIONAL_INFO: u8 = 0x10;
    pub const INNER_STATUS_CODE: u8 = 0x20;
    pub const INNER_DIAGNOSTIC_INFO: u8 = 0x40;
}

impl Serialize for DiagnosticInfo {
    fn serialize(&self) -> Vec<u8> {
        let mut result = vec![0x00]; //temporary null encoding mask
        if self.symbolic_id != -1 {
            result[0] = result[0] | EncodingValue::SYMBOLIC_ID;
            result.extend_from_slice(&self.symbolic_id.serialize());
        }
        if self.namespace_uri != -1 {
            result[0] = result[0] | EncodingValue::NAMESPACE;
            result.extend_from_slice(&self.namespace_uri.serialize());
        }
        if self.locale != -1 {
            result[0] = result[0] | EncodingValue::LOCALIZED_TEXT;
            result.extend_from_slice(&self.locale.serialize());
        }
        if self.localized_text != -1 {
            result[0] = result[0] | EncodingValue::LOCALE;
            result.extend_from_slice(&self.localized_text.serialize());
        }
        if !self.additional_info.isnull() {
            result[0] = result[0] | EncodingValue::ADDITIONAL_INFO;
            result.extend_from_slice(&self.additional_info.serialize());
        }
        if StatusCode::is_status_code(&self.inner_status_code) {
            result[0] = result[0] | EncodingValue::INNER_STATUS_CODE;
            result.extend_from_slice(&self.additional_info.serialize());
        }
        if let Some(inner_diagnostic_info) = &self.inner_diagnostic_info {
            result[0] = result[0] | EncodingValue::INNER_DIAGNOSTIC_INFO;
            result.extend_from_slice(&inner_diagnostic_info.serialize());
        }
        result
    }
}

impl Deserialize for DiagnosticInfo {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (mut data, encoding_mask) = u8::deserialize(data)?;
        let mut symbolic_id: i32 = -1;
        let mut namespace_uri: i32 = -1;
        let mut locale: i32 = -1;
        let mut localized_text: i32 = -1;
        let mut additional_info = UaString::new();
        let mut inner_status_code: u32 = 0xFFFFFFFF;
        let mut inner_diagnostic_info: Option<Box<DiagnosticInfo>> = None;
        if encoding_mask & EncodingValue::SYMBOLIC_ID == EncodingValue::SYMBOLIC_ID {
            (data, symbolic_id) = i32::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::NAMESPACE == EncodingValue::NAMESPACE {
            (data, namespace_uri) = i32::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::LOCALE == EncodingValue::LOCALE {
            (data, localized_text) = i32::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::LOCALIZED_TEXT == EncodingValue::LOCALIZED_TEXT {
            (data, locale) = i32::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::ADDITIONAL_INFO == EncodingValue::ADDITIONAL_INFO {
            (data, additional_info) = UaString::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::INNER_STATUS_CODE == EncodingValue::INNER_STATUS_CODE {
            (data, inner_status_code) = u32::deserialize(data)?;
        }
        if encoding_mask & EncodingValue::INNER_DIAGNOSTIC_INFO
            == EncodingValue::INNER_DIAGNOSTIC_INFO
        {
            let (data_, inner_diagnostic) = DiagnosticInfo::deserialize(data)?;
            data = data_;
            inner_diagnostic_info = Some(Box::new(inner_diagnostic));
        }
        Ok((
            data,
            DiagnosticInfo {
                encoding_mask,
                symbolic_id,
                namespace_uri,
                locale,
                localized_text,
                additional_info,
                inner_status_code,
                inner_diagnostic_info,
            },
        ))
    }
}

impl DiagnosticInfo {
    pub fn new() -> DiagnosticInfo {
        DiagnosticInfo {
            encoding_mask: 0,
            symbolic_id: -1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0,
            inner_diagnostic_info: None,
        }
    }
}
