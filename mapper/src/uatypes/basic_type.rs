// OPC UA Part 6 - Mappings 1.03 Specification

// Standard UA types onto Rust types:

use crate::{Deserialize, MapperResult};

// Boolean  -> bool
// SByte    -> i8
// Byte     -> u8
// Int16    -> i16
// UInt16   -> u16
// Int32    -> i32
// UInt32   -> u32
// Int64    -> i64
// UInt64   -> u64
// Float    -> f32
// Double   -> f64
use super::{super::Serialize, ToVariant, prelude::Variant};

impl Serialize for u8 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for u8 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u8::take_count(data, 1)?;
        let value = u8::from_le_bytes(*arrayref::array_ref!(value, 0, 1));
        Ok((data, value))
    }
}

impl ToVariant for u8{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Byte(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for u16 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}


impl Deserialize for u16 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u16::take_count(data, 2)?;
        let value = u16::from_le_bytes(*arrayref::array_ref!(value, 0, 2));
        Ok((data,value))
    }
}

impl ToVariant for u16{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Uint16(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for u32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for u32 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u32::take_count(data, 4)?;
        let value = u32::from_le_bytes(*arrayref::array_ref!(value, 0, 4));
        Ok((data,value))
    }
}

impl ToVariant for u32{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Uint32(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for u64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for u64 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u64::take_count(data, 8)?;
        let value = u64::from_le_bytes(*arrayref::array_ref!(value, 0, 8));
        Ok((data,value))
    }
}

impl ToVariant for u64{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Uint64(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for i8 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}


impl Deserialize for i8 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u8::take_count(data, 1)?;
        let value = i8::from_le_bytes(*arrayref::array_ref!(value, 0, 1));
        Ok((data,value))
    }
}

impl ToVariant for i8{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Sbyte(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for i16 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i16 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i16::take_count(data, 2)?;
        let value = i16::from_le_bytes(*arrayref::array_ref!(value, 0, 2));
        Ok((data,value))
    }
}

impl ToVariant for i16{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Int16(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i32 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i32::take_count(data, 4)?;
        let value = i32::from_le_bytes(*arrayref::array_ref!(value, 0, 4));
        Ok((data,value))
    }
}

impl ToVariant for i32{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Int32(self.clone())],
                array_dimension:vec![],
            }
    }
}


impl Serialize for i64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for i64 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i64::take_count(data, 8)?;
        let value = i64::from_le_bytes(*arrayref::array_ref!(value, 0, 8));
        Ok((data,value))
    }
}

impl ToVariant for i64{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Int64(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for bool {
    fn serialize(&self) -> Vec<u8> {
        if *self {
            vec![1]
        } else {
            vec![0]
        }
    }
}

impl Deserialize for bool {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = i8::take_count(data, 1)?;
        let value = i8::from_le_bytes(*arrayref::array_ref!(value, 0, 1));
        if value == 0 {
            Ok((data, false))
        } else {
            Ok((data, true))
        }
    }
}

impl ToVariant for bool{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Boolean(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for f32 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for f32 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = f32::take_count(data, 4)?;
        let value = f32::from_le_bytes(*arrayref::array_ref!(value, 0, 4));
        Ok((data, value))
    }
}

impl ToVariant for f32{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Float(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for f64 {
    fn serialize(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Deserialize for f64 {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = f64::take_count(data, 8)?;
        let value = f64::from_le_bytes(*arrayref::array_ref!(value, 0, 8));
        Ok((data, value))
    }
}

impl ToVariant for f64{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::Double(self.clone())],
                array_dimension:vec![],
            }
    }
}

impl Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let mut serialization = (self.len() as u32).to_le_bytes().to_vec();
        serialization.extend_from_slice(self.as_bytes());
        serialization
    }
}

impl Deserialize for String {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, value) = u32::deserialize(data)?;
        let (data, value) = String::take_count(data, value as usize)?;
        let value = String::from_utf8(value).unwrap();
        Ok((data, value))
    }
}

impl ToVariant for String{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::String(super::string::UaString::from(&self))],
                array_dimension:vec![],
            }
    }
}

impl ToVariant for &str{
    fn to_variant(&self)->super::prelude::Variant {
        Variant{value:vec![super::prelude::Value::String(super::string::UaString::from(&self))],
                array_dimension:vec![],
            }
    }
}

impl<T> Serialize for Vec<T>
where
    T: Serialize,
{
    fn serialize(&self) -> Vec<u8> {
        let mut result = (self.len() as i32).serialize();
        for i in self {
            result.extend_from_slice(&i.serialize());
        }
        result
    }
}

impl<T> Deserialize for Vec<T>
where
    T: Deserialize,
{
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (mut data, len) = i32::deserialize(data)?;
        if len==-1{
            return Ok((data, Vec::new()))
        }
        let mut vector = Vec::with_capacity(len as usize);
        let mut el: T;
        for _i in 0..len {
            (data, el) = T::deserialize(data)?;
            vector.push(el);
        }
        Ok((data, vector))
    }
}

