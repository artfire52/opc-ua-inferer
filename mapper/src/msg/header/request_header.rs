use crate::{
    uatypes::{date_time::DateTime, node_id::NodeId, string::UaString},
    Deserialize, Serialize, MapperResult,
};

use super::additional_header::AdditionalHeaders;

#[derive(Debug)]
pub (crate)  struct RequestHeader {
    pub(crate) authentication_token: NodeId,
    pub(crate) timestamp: DateTime,
    pub(crate) request_handle: u32,
    pub(crate) return_diagnostic: u32,
    pub(crate) audit_entry: UaString,
    pub(crate) timout_hint: u32,
    pub(crate) additional_header: Option<AdditionalHeaders>,
}

impl Serialize for RequestHeader {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.authentication_token.serialize();
        result.extend_from_slice(&self.timestamp.serialize());
        result.extend_from_slice(&self.request_handle.serialize());
        result.extend_from_slice(&self.return_diagnostic.serialize());
        result.extend_from_slice(&self.audit_entry.serialize());
        result.extend_from_slice(&self.timout_hint.serialize());
        match &self.additional_header {
            None => result.extend_from_slice(&[0; 3]), //no additional header we add an empty one
            Some(add_headers) => result.extend_from_slice(&add_headers.serialize()),
        }

        result
    }
}
//we ignore additional headers for now but de not derive because we will cop with additionnal headers
impl Deserialize for RequestHeader {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, authentication_token) = NodeId::deserialize(data)?;
        let (data, timestamp) = DateTime::deserialize(data)?;
        let (data, request_handle) = u32::deserialize(data)?;
        let (data, return_diagnostic) = u32::deserialize(data)?;
        let (data, audit_entry) = UaString::deserialize(data)?;
        let (data, timout_hint) = u32::deserialize(data)?;
        let (data, _) = RequestHeader::take_count(data, 3)?;
        Ok((
            data,
            RequestHeader {
                authentication_token,
                timestamp,
                request_handle,
                return_diagnostic,
                audit_entry,
                timout_hint,
                additional_header: None,
            },
        ))
    }
}
