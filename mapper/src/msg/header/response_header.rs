use crate::{
    uatypes::{
        date_time::DateTime, diagnostic_info::DiagnosticInfo, extension_object::ExtensionObject,
        status_code::StatusCode, string::UaString,
    },
    Deserialize, Serialize, MapperResult,
};

#[derive(Debug)]

pub (crate)  struct ResponseHeader {
    pub(crate) timestamp: DateTime,
    pub(crate) request_handle: u32,
    pub(crate) service_result: StatusCode,
    pub(crate) service_diagnostic: DiagnosticInfo,
    pub(crate) string_table: UaString,
    pub(crate) additional_header: ExtensionObject,
}

impl Serialize for ResponseHeader {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.timestamp.serialize();
        result.extend_from_slice(&self.request_handle.serialize());
        result.extend_from_slice(&self.service_result.serialize());
        result.extend_from_slice(&self.service_diagnostic.serialize());
        result.extend_from_slice(&self.string_table.serialize());
        result
    }
}

impl Deserialize for ResponseHeader {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, timestamp) = DateTime::deserialize(data)?;
        let (data, request_handle) = u32::deserialize(data)?;
        let (data, service_result) = StatusCode::deserialize(data)?;
        let (data, service_diagnostic) = DiagnosticInfo::deserialize(data)?;
        let (data, string_table) = UaString::deserialize(data)?;
        let (data, additional_header) = ExtensionObject::deserialize(data)?;
        Ok((
            data,
            ResponseHeader {
                timestamp,
                request_handle,
                service_result,
                service_diagnostic,
                string_table,
                additional_header,
            },
        ))
    }
}
