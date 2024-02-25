use crate::Deserialize;

use crate::Serialize;
use crate::MapperResult;

#[derive(PartialEq, Debug, Clone)]
pub struct ByteString {
    pub(crate) value: Option<Vec<u8>>,
}

impl Serialize for ByteString {
    fn serialize(&self) -> Vec<u8> {
        match &self.value {
            Some(s) => {
                let mut serialization = (s.len() as i32).to_le_bytes().to_vec();
                serialization.extend_from_slice(s);
                serialization
            }
            _ => {
                let serialization = (-1 as i32).serialize();
                serialization
            }
        }
    }
}

impl Deserialize for ByteString {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i32::deserialize(data)?;
        if value < 0 {
            Ok((data, ByteString::new()))
        } else {
            let (data, value) = ByteString::take_count(data, value as usize)?;
            let value = ByteString::from(value);
            Ok((data, value))
        }
    }
}

impl ByteString {
    pub fn new() -> ByteString {
        ByteString { value: None }
    }
    pub fn from_str(s: &str) -> ByteString {
        ByteString {
            value: Some(Vec::from(s)),
        }
    }
    pub fn from(vec: Vec<u8>) -> ByteString {
        ByteString { value: Some(vec) }
    }
    pub fn isnull(&self) -> bool {
        self.value.is_none()
    }

    pub fn append_byte_string(&mut self,b:&ByteString){
        match (&mut self.value,&b.value){
            (_,None)=>{},
            (None,Some(_))=>*self=b.clone(),
            (Some(self_v),Some(b_v))=>self_v.extend_from_slice(b_v),
        }

    }
    pub fn append_slice(&mut self,b:&[u8]){
        match &mut self.value{
            None=>{self.value=Some(b.to_vec());},
            Some(self_v)=>self_v.extend_from_slice(b),
        }

    }

}
