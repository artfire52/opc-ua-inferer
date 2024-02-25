use std::str::FromStr;

use crate::encoding_prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq,Clone)]
pub struct Guid {
    uuid: Uuid,
}

impl Guid {
    pub fn new() -> Guid {
        Guid {
            uuid: Uuid::new_v4(),
        }
    }
    pub fn from_uuid(guid: Uuid) -> Guid {
        Guid { uuid: guid }
    }
    pub fn from(guid: &str) -> Guid {
        Guid {
            uuid: Uuid::from_str(guid).unwrap(),
        }
    }
}

impl Serialize for Guid {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        let (field1, field2, field3, field4) = self.uuid.as_fields();
        result.extend_from_slice(&field1.serialize());
        result.extend_from_slice(&field2.serialize());
        result.extend_from_slice(&field3.serialize());
        result.extend_from_slice(field4);
        result
    }
}

impl Deserialize for Guid {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, field1) = u32::deserialize(data)?;
        let (data, field2) = u16::deserialize(data)?;
        let (data, field3) = u16::deserialize(data)?;
        let (data, field4) = Guid::take_count(data, 8)?;
        let field4 = arrayref::array_ref!(field4, 0, 8);
        let uuid = uuid::Builder::from_fields(field1, field2, field3, &field4).into_uuid();
        Ok((data, Guid { uuid }))
    }
}
