use super::byte_string::ByteString;
use super::guid::Guid;
use super::string::UaString;
use crate::encoding_prelude::*;
use crate::result_prelude::{*};

#[derive(Debug, PartialEq,Clone)]
pub struct NodeId {
    pub(crate) namespace: u16,
    pub(crate) identifier: Identifier,
}

#[derive(Debug, PartialEq,Clone)]
pub enum Identifier {
    Numeric(u32),
    String(UaString),
    Guid(Guid),
    ByteString(ByteString),
}

impl Serialize for NodeId {
    fn serialize(&self) -> Vec<u8> {
        match &self.identifier {
            Identifier::Numeric(i) => {
                if *i < 255 && self.namespace == 0 {
                    let mut result = vec![EncodingValue::TWO_BYTE];
                    result.extend_from_slice(&(*i as u8).serialize());
                    result
                } else if *i < 65535 {
                    let mut result = vec![EncodingValue::FOUR_BYTE];
                    result.extend_from_slice(&(self.namespace as u8).serialize());
                    result.extend_from_slice(&(*i as u16).serialize());
                    result
                } else {
                    let mut result = vec![EncodingValue::NUMERIC];
                    result.extend_from_slice(&self.namespace.serialize());
                    result.extend_from_slice(&i.serialize());
                    result
                }
            }
            Identifier::String(s) => {
                let mut result = vec![EncodingValue::STRING];
                result.extend_from_slice(&self.namespace.serialize());
                result.extend_from_slice(&s.serialize());
                result
            }
            Identifier::Guid(guid) => {
                let mut result = vec![EncodingValue::GUID];
                result.extend_from_slice(&self.namespace.serialize());
                result.extend_from_slice(&guid.serialize());
                result
            }
            Identifier::ByteString(b) => {
                let mut result = vec![EncodingValue::BYTE_STRING];
                result.extend_from_slice(&self.namespace.serialize());
                result.extend_from_slice(&b.serialize());
                result
            }
        }
    }
}

impl Deserialize for NodeId {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, encoding) = u8::deserialize(data)?;
        match encoding {
            EncodingValue::TWO_BYTE => {
                let (data, numeric) = u8::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace: 0,
                        identifier: Identifier::Numeric(numeric as u32),
                    },
                ))
            }
            EncodingValue::FOUR_BYTE => {
                let (data, namespace) = u8::deserialize(data)?;
                let (data, numeric) = u16::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace: namespace as u16,
                        identifier: Identifier::Numeric(numeric as u32),
                    },
                ))
            }
            EncodingValue::NUMERIC => {
                let (data, namespace) = u16::deserialize(data)?;
                let (data, numeric) = u32::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace,
                        identifier: Identifier::Numeric(numeric),
                    },
                ))
            }
            EncodingValue::GUID => {
                let (data, namespace) = u16::deserialize(data)?;
                let (data, g) = Guid::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace,
                        identifier: Identifier::Guid(g),
                    },
                ))
            }
            EncodingValue::BYTE_STRING => {
                let (data, namespace) = u16::deserialize(data)?;
                let (data, s) = ByteString::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace,
                        identifier: Identifier::ByteString(s),
                    },
                ))
            }
            EncodingValue::STRING => {
                let (data, namespace) = u16::deserialize(data)?;
                let (data, s) = UaString::deserialize(data)?;
                Ok((
                    data,
                    NodeId {
                        namespace,
                        identifier: Identifier::String(s),
                    },
                ))
            }

            _ => Err(MapperError::new(MapperErrorKind::ParsingError,"node id parsing failed")),
        }
    }
}

impl NodeId {
    pub fn new_numeric(namespace: u16, id: u32) -> NodeId {
        NodeId {
            namespace,
            identifier: Identifier::Numeric(id),
        }
    }
    pub fn new_string(namespace: u16, id: UaString) -> NodeId {
        NodeId {
            namespace,
            identifier: Identifier::String(id),
        }
    }
    pub fn new_guid(namespace: u16, id: Guid) -> NodeId {
        NodeId {
            namespace,
            identifier: Identifier::Guid(id),
        }
    }
    pub fn new_bytestring(namespace: u16, id: ByteString) -> NodeId {
        NodeId {
            namespace,
            identifier: Identifier::ByteString(id),
        }
    }
    pub fn new(namespace:u16,id:Identifier)->NodeId{
        match id{
            Identifier::Numeric(i)=> {
                    NodeId::new_numeric(namespace, i)
            },
            Identifier::String(i)=> {
                NodeId::new_string(namespace, i)
            },
            Identifier::Guid(i)=> {
                NodeId::new_guid(namespace, i)
            },
            Identifier::ByteString(i)=> {
                NodeId::new_bytestring(namespace, i)
            },
        }
    }
    
    pub fn from_str_to_id(s:&str,type_:u8)->Identifier{
        match type_{
            EncodingValue::TWO_BYTE | EncodingValue::FOUR_BYTE | EncodingValue::NUMERIC=>{
                let id=s.parse::<u32>().expect("expected u32 as NodeId Identifier with option numeric");
                Identifier::Numeric(id)
                
            },
            EncodingValue::STRING=>{
                Identifier::String(UaString::from(s))
            },
            EncodingValue::GUID=>{
                Identifier::Guid(Guid::from(s))
            },
            EncodingValue::BYTE_STRING=>{
                Identifier::ByteString(ByteString::from_str(s))
            },
            _=>{panic!("Unrecognize node id identifier type")}
        }
    }

    
    pub fn empty() -> NodeId {
        NodeId::new_numeric(0, 0)
    }
}
pub struct EncodingValue;
impl EncodingValue {
    pub const TWO_BYTE: u8 = 0x00;
    pub const FOUR_BYTE: u8 = 0x01;
    pub const NUMERIC: u8 = 0x02;
    pub const STRING: u8 = 0x03;
    pub const GUID: u8 = 0x04;
    pub const BYTE_STRING: u8 = 0x05;
    pub const NAMSPACE_URI: u8 = 0x80;
    pub const SERVER_INDEX: u8 = 0x40;
}
