use crate::{Deserialize, Serialize, MapperResult};

use super::date_time::DateTime;
use super::variant::Variant;

#[derive(Debug,Clone)]
pub struct DataValue {
    pub(crate) value: Option<Variant>,
    pub(crate) status: Option<u32>,
    pub(crate) source_time_stamp: Option<DateTime>,
    pub(crate) source_pico_seconds: Option<u16>,
    pub(crate) server_time_stamp: Option<DateTime>,
    pub(crate) server_pico_seconds: Option<u16>,
}
pub struct EncodingValue;
impl EncodingValue {
    pub const HAS_VALUE: u8 = 0x01;
    pub const STATUS_CODE: u8 = 0x02;
    pub const SOURCE_TIME_STAMP: u8 = 0x04;
    pub const SERVER_TIME_STAMP: u8 = 0x08;
    pub const SOURCE_PICO_SECONDS: u8 = 0x10;
    pub const SERVER_PICO_SECONDS: u8 = 0x20;
}

impl Serialize for DataValue
{
    fn serialize(&self) -> Vec<u8> {
        let mut result = vec![0x00];
        if let Some(v)=&self.value {
            result[0]=result[0]| EncodingValue::HAS_VALUE;
            result.extend_from_slice(&v.serialize());
        }
        if let Some(v)=&self.status {
            result[0]=result[0]| EncodingValue::STATUS_CODE;
            result.extend_from_slice(&v.serialize());
        }
        if let Some(v)=&self.source_time_stamp {
            result[0]=result[0]| EncodingValue::SOURCE_TIME_STAMP;
            result.extend_from_slice(&v.serialize());
        }
        if let Some(v)=&self.source_pico_seconds {
            result[0]=result[0]| EncodingValue::SOURCE_PICO_SECONDS;
            result.extend_from_slice(&v.serialize());
        }
        if let Some(v)=&self.server_time_stamp {
            result[0]=result[0]| EncodingValue::SERVER_TIME_STAMP;
            result.extend_from_slice(&v.serialize());
        }
        if let Some(v)=&self.server_pico_seconds {
            result[0]=result[0]| EncodingValue::SERVER_PICO_SECONDS;
            result.extend_from_slice(&v.serialize());
        }
        result
    }
}

impl Deserialize for DataValue
{
    fn deserialize(data: &[u8]) -> crate::MapperResult<(&[u8], Self)> where Self: Sized {
        let mut value= None;
        let mut status= None;
        let mut source_time_stamp= None;
        let mut source_pico_seconds= None;
        let mut server_time_stamp= None;
        let mut server_pico_seconds= None;
        let (mut data,encoding_value)=u8::deserialize(data)?;
        if encoding_value & EncodingValue::HAS_VALUE == EncodingValue::HAS_VALUE  {

            let val: Variant;
            (data,val)=Variant::deserialize(data)?;
            value=Some(val);
        }
        if encoding_value & EncodingValue::STATUS_CODE == EncodingValue::STATUS_CODE  {
            let val: u32;
            (data,val)=u32::deserialize(data)?;
            status=Some(val);
        }
        if encoding_value & EncodingValue::SOURCE_TIME_STAMP == EncodingValue::SOURCE_TIME_STAMP  {
            let val: DateTime;
            (data,val)=DateTime::deserialize(data)?;
            source_time_stamp=Some(val);
        }
        if encoding_value & EncodingValue::SOURCE_PICO_SECONDS == EncodingValue::SOURCE_PICO_SECONDS  {
            let val: u16;
            (data,val)=u16::deserialize(data)?;
            source_pico_seconds=Some(val);
        }
        if encoding_value & EncodingValue::SERVER_TIME_STAMP == EncodingValue::SERVER_TIME_STAMP  {
            let val: DateTime;
            (data,val)=DateTime::deserialize(data)?;
            server_time_stamp=Some(val);
        }
        if encoding_value & EncodingValue::SERVER_PICO_SECONDS == EncodingValue::SERVER_PICO_SECONDS  {
            let val: u16;
            (data,val)=u16::deserialize(data)?;
            server_pico_seconds=Some(val);
        }
        Ok((data,DataValue {
                            value,
                            status,
                            source_time_stamp,
                            source_pico_seconds,
                            server_time_stamp,
                            server_pico_seconds,
        }))

    }
}

impl DataValue{
    pub(crate) fn empty()->Self{
        DataValue {
            value: None,
            status: None,
            source_time_stamp: None,
            source_pico_seconds: None,
            server_time_stamp: None,
            server_pico_seconds: None,
        }
    }
    pub(crate) fn from_value(type_:u8,value:&str)->MapperResult<Self>{
        Ok(DataValue {
            value: Some(Variant::from_datatype(type_, value)?),
            status: None,
            source_time_stamp: None,
            source_pico_seconds: None,
            server_time_stamp: None,
            server_pico_seconds: None,
        })
    }
}