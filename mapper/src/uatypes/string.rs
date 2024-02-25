use crate::encoding_prelude::*;

//String need to be able to deal with null. So we need to wrap the String into an Option.
#[derive(PartialEq, Debug,Clone)]
pub struct UaString {
    pub(crate) value: Option<String>,
}

impl Serialize for UaString {
    fn serialize(&self) -> Vec<u8> {
        match &self.value {
            Some(s) if s.len()==0 => {
                let mut serialization = (s.len() as i32).to_le_bytes().to_vec();
                serialization.extend_from_slice(s.as_bytes());
                serialization
            }
            Some(s) => {
                let mut serialization = (s.len() as i32).to_le_bytes().to_vec();
                serialization.extend_from_slice(s.as_bytes());
                serialization
            }
            _ => {
                let serialization = (-1 as i32).serialize();
                serialization
            }
        }
    }
}

impl Deserialize for UaString {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)>{
        let (data, value) = i32::deserialize(data)?;
        if value < 0 {
            Ok((data, UaString::new()))
        } else {
            let (data, value) = UaString::take_count(data, value as usize)?;
            let value = UaString::from_utf8(value);
            Ok((data, value))
        }
    }
}

impl UaString {
    pub fn new() -> UaString {
        UaString { value: None }
    }
    pub fn from(s: &str) -> UaString {
        UaString {
            value: Some(String::from(s)),
        }
    }
    pub fn from_utf8(s: Vec<u8>) -> UaString {
        UaString {
            value: Some(String::from_utf8(s).unwrap()),
        }
    }
    pub fn value(&self) -> &Option<String> {
        &self.value
    }

    pub fn isnull(&self) -> bool {
        self.value.is_none()
    }

    pub fn contain(&self,s:&str) -> bool {
        if self.value.is_none(){
            return false;
        }
        self.value.as_ref().unwrap().contains(s)
    }
}
