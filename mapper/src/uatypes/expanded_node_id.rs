use super::node_id::{EncodingValue, NodeId};
use super::string::UaString;
use crate::encoding_prelude::*;

#[derive(Debug,Clone)]
pub struct ExpandedNodeId {
    nodeid: NodeId,
    namespace_uri: UaString,
    server_index: u32,
}

impl Serialize for ExpandedNodeId {
    fn serialize(&self) -> Vec<u8> {
        match (self.namespace_uri.value(), self.server_index) {
            (Some(s), x) if x > 0 => {
                let mut result = self.nodeid.serialize();
                result[0] = result[0] | EncodingValue::NAMSPACE_URI | EncodingValue::SERVER_INDEX;
                result.extend_from_slice(&s.serialize());
                result.extend_from_slice(&x.serialize());
                result
            }
            (Some(s), 0) => {
                let mut result = self.nodeid.serialize();
                result[0] = result[0] | EncodingValue::NAMSPACE_URI;
                result.extend_from_slice(&s.serialize());
                result
            }
            (None, x) if x > 0 => {
                let mut result = self.nodeid.serialize();
                result[0] = result[0] | EncodingValue::SERVER_INDEX;
                result.extend_from_slice(&x.serialize());
                result
            }
            _ => Vec::new(),
        }
    }
}

impl Deserialize for ExpandedNodeId {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)>  {
        let (_, encoding)=u8::deserialize(data)?;
        let (mut data, node_id)=NodeId::deserialize(data)?;
        let mut namespace_uri=UaString::new();
        let mut server_index=0;
        if encoding & EncodingValue::NAMSPACE_URI == EncodingValue::NAMSPACE_URI{
            (data, namespace_uri)=UaString::deserialize(data)?;
        }
        if encoding & EncodingValue::SERVER_INDEX == EncodingValue::SERVER_INDEX{
            (data, server_index)=u32::deserialize(data)?;
        }
        Ok((data,ExpandedNodeId{
            nodeid:node_id,
            namespace_uri: namespace_uri,
            server_index: server_index
        }))
    }
}
