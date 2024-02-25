use crate::{encoding_prelude::*, result_prelude::*};
use super::prelude::*;

#[derive(Debug,Clone)]
//Variant is a structure to contain vector of opc ua datatype
pub struct Variant {
    pub(crate) value: Vec<Value>,
    pub(crate) array_dimension: Vec<i32>,
}

#[derive(Serialize,Debug,Clone)]

pub(crate) enum Value{
    Boolean(bool),
    Sbyte(i8),
    Byte(u8),
    Int16(i16),
    Uint16(u16),
    Int32(i32),
    Uint32(u32),
    Int64(i64),
    Uint64(u64),
    Float(f32),
    Double(f64),
    String(UaString),
    DateTime(DateTime),
    Guid(Guid),
    ByteString(ByteString),
    XmlElement(XmlElement),
    NodeId(NodeId),
    ExpandedNodeId(ExpandedNodeId),
    StatusCode(u32),
    QualifiedName(QualifiedName),
    LocaizedText(LocalizedText),
    ExtensionObject(ExtensionObject),
    DataValue(DataValue),
    Variant(Box<Variant>),
    DiagnosticInfo(DiagnosticInfo)

}
impl Value{
    pub(crate) fn get_encoding_value(val:&Value)->u8{
        match val{
            Value::Boolean(_)=> DataTypeId::BOOLEAN,
            Value::Sbyte(_)=>DataTypeId::SBYTE,
            Value::Byte(_)=>DataTypeId::BYTE,
            Value::Int16(_)=>DataTypeId::INT_16,
            Value::Uint16(_)=>DataTypeId::UINT_16,
            Value::Int32(_)=>DataTypeId::INT_32,
            Value::Uint32(_)=>DataTypeId::UINT_32,
            Value::Int64(_)=>DataTypeId::INT_64,
            Value::Uint64(_)=>DataTypeId::UINT_64,
            Value::Float(_)=>DataTypeId::FLOAT,
            Value::Double(_)=>DataTypeId::DOUBLE,
            Value::String(_)=>DataTypeId::STRING,
            Value::DateTime(_)=>DataTypeId::DATETIME,
            Value::Guid(_)=>DataTypeId::GUID,
            Value::ByteString(_)=>DataTypeId::BYTESTRING,
            Value::XmlElement(_)=>DataTypeId::XMLELEMENT,
            Value::NodeId(_)=>DataTypeId::NODEID,
            Value::ExpandedNodeId(_)=>DataTypeId::EXPANDED_NODEID,
            Value::StatusCode(_)=>DataTypeId::STATUS_CODE,
            Value::QualifiedName(_)=>DataTypeId::QUALIFIE_NAME,
            Value::LocaizedText(_)=>DataTypeId::LOCALIZED_TEXT,
            Value::ExtensionObject(_)=>DataTypeId::EXTENSION_OBJECT,
            Value::DataValue(_)=>DataTypeId::DATAVALUE,
            Value::Variant(_)=>DataTypeId::VARIANT,
            Value::DiagnosticInfo(_)=>DataTypeId::DIAGNOSTIC_INFO,
        }
    }
}

impl Serialize for Variant
{
    fn serialize(&self) -> Vec<u8> {
        let mut result = vec![0x00];
        if self.value.len()==0 {
            return result;
        }
        else if self.value.len()==1 
        {
            result[0]=Value::get_encoding_value(&self.value[0]);
            result.extend_from_slice(&self.value[0].serialize());
        }
        else
        {
            result[0]=result[0]|Value::get_encoding_value(&self.value[0])|DataTypeId::ARRAY_VALUE_ENCODED;
            result.extend_from_slice(&self.value.serialize());
        }
        //We currently ignore multidimensionnal array
        result
    }
}

impl Deserialize for Variant

{
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (mut data, encoding_mask) = u8::deserialize(data)?;
        if encoding_mask == DataTypeId::NULL {
            return Ok((
                data,
                Variant {
                    value:vec![],
                    array_dimension:vec![],
                },
            ));
        }
        let mut value:Vec<Value>=vec![];
        let mut array_dimension : Vec<i32>=vec![];
        if encoding_mask & 0x3f == DataTypeId::BOOLEAN {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<bool>;
                (data, val) = Vec::<bool>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Boolean(el)}).collect();
            }
            else {
                let val : bool;
                (data, val) = bool::deserialize(data)?;
                value=vec![Value::Boolean(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::SBYTE {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<i8>;
                (data, val) = Vec::<i8>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Sbyte(el)}).collect();
            }
            else {
                let val : i8;
                (data, val) = i8::deserialize(data)?;
                value=vec![Value::Sbyte(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::BYTE {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<u8>;
                (data, val) = Vec::<u8>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Byte(el)}).collect();
            }
            else {
                let val : u8;
                (data, val) = u8::deserialize(data)?;
                value=vec![Value::Byte(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::INT_16 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<i16>;
                (data, val) = Vec::<i16>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Int16(el)}).collect();
            }
            else {
                let val : i16;
                (data, val) = i16::deserialize(data)?;
                value=vec![Value::Int16(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::UINT_16 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<u16>;
                (data, val) = Vec::<u16>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Uint16(el)}).collect();
            }
            else {
                let val : u16;
                (data, val) = u16::deserialize(data)?;
                value=vec![Value::Uint16(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::INT_32 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<i32>;
                (data, val) = Vec::<i32>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Int32(el)}).collect();
            }
            else {
                let val : i32;
                (data, val) = i32::deserialize(data)?;
                value=vec![Value::Int32(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::UINT_32 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<u32>;
                (data, val) = Vec::<u32>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Uint32(el)}).collect();
            }
            else {
                let val : u32;
                (data, val) = u32::deserialize(data)?;
                value=vec![Value::Uint32(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::INT_64 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<i64>;
                (data, val) = Vec::<i64>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Int64(el)}).collect();
            }
            else {
                let val : i64;
                (data, val) = i64::deserialize(data)?;
                value=vec![Value::Int64(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::UINT_64 {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<u64>;
                (data, val) = Vec::<u64>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Uint64(el)}).collect();
            }
            else {
                let val : u64;
                (data, val) = u64::deserialize(data)?;
                value=vec![Value::Uint64(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::FLOAT {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<f32>;
                (data, val) = Vec::<f32>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Float(el)}).collect();
            }
            else {
                let val : f32;
                (data, val) = f32::deserialize(data)?;
                value=vec![Value::Float(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::DOUBLE {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<f64>;
                (data, val) = Vec::<f64>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Double(el)}).collect();
            }
            else {
                let val : f64;
                (data, val) = f64::deserialize(data)?;
                value=vec![Value::Double(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::STRING {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<UaString>;
                (data, val) = Vec::<UaString>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::String(el)}).collect();
            }
            else {
                let val : UaString;
                (data, val) = UaString::deserialize(data)?;
                value=vec![Value::String(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::DATETIME {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<DateTime>;
                (data, val) = Vec::<DateTime>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::DateTime(el)}).collect();
            }
            else {
                let val : DateTime;
                (data, val) = DateTime::deserialize(data)?;
                value=vec![Value::DateTime(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::GUID {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<Guid>;
                (data, val) = Vec::<Guid>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::Guid(el)}).collect();
            }
            else {
                let val : Guid;
                (data, val) = Guid::deserialize(data)?;
                value=vec![Value::Guid(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::BYTESTRING {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<ByteString>;
                (data, val) = Vec::<ByteString>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::ByteString(el)}).collect();
            }
            else {
                let val : ByteString;
                (data, val) = ByteString::deserialize(data)?;
                value=vec![Value::ByteString(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::XMLELEMENT {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<XmlElement>;
                (data, val) = Vec::<XmlElement>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::XmlElement(el)}).collect();
            }
            else {
                let val : XmlElement;
                (data, val) = XmlElement::deserialize(data)?;
                value=vec![Value::XmlElement(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::NODEID {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<NodeId>;
                (data, val) = Vec::<NodeId>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::NodeId(el)}).collect();
            }
            else {
                let val : NodeId;
                (data, val) = NodeId::deserialize(data)?;
                value=vec![Value::NodeId(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::EXPANDED_NODEID {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<ExpandedNodeId>;
                (data, val) = Vec::<ExpandedNodeId>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::ExpandedNodeId(el)}).collect();
            }
            else {
                let val : ExpandedNodeId;
                (data, val) = ExpandedNodeId::deserialize(data)?;
                value=vec![Value::ExpandedNodeId(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::STATUS_CODE {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<u32>;
                (data, val) = Vec::<u32>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::StatusCode(el)}).collect();
            }
            else {
                let val : u32;
                (data, val) = u32::deserialize(data)?;
                value=vec![Value::StatusCode(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::QUALIFIE_NAME {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<QualifiedName>;
                (data, val) = Vec::<QualifiedName>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::QualifiedName(el)}).collect();
            }
            else {
                let val : QualifiedName;
                (data, val) = QualifiedName::deserialize(data)?;
                value=vec![Value::QualifiedName(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::LOCALIZED_TEXT {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<LocalizedText>;
                (data, val) = Vec::<LocalizedText>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::LocaizedText(el)}).collect();
            }
            else {
                let val : LocalizedText;
                (data, val) = LocalizedText::deserialize(data)?;
                value=vec![Value::LocaizedText(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::EXTENSION_OBJECT {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<ExtensionObject>;
                (data, val) = Vec::<ExtensionObject>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::ExtensionObject(el)}).collect();
            }
            else {
                let val : ExtensionObject;
                (data, val) = ExtensionObject::deserialize(data)?;
                value=vec![Value::ExtensionObject(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::DATAVALUE {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<DataValue>;
                (data, val) = Vec::<DataValue>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::DataValue(el)}).collect();
            }
            else {
                let val : DataValue;
                (data, val) = DataValue::deserialize(data)?;
                value=vec![Value::DataValue(val)];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::VARIANT {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<Variant>;
                (data, val) = Vec::<Variant>::deserialize(data)?;
                value =val.into_iter().map(|el| {Value::Variant(Box::new(el))}).collect();
            }
            else {
                let val : Variant;
                (data, val) = Variant::deserialize(data)?;
                value=vec![Value::Variant(Box::new(val))];
            }
        }
        if encoding_mask & 0x3f == DataTypeId::DIAGNOSTIC_INFO {
            if encoding_mask &DataTypeId::ARRAY_VALUE_ENCODED == DataTypeId::ARRAY_VALUE_ENCODED {
                let val : Vec<DiagnosticInfo>;
                (data, val) = Vec::<DiagnosticInfo>::deserialize(data)?;
                value=val.into_iter().map(|el| {Value::DiagnosticInfo(el)}).collect();
            }
            else {
                let val : DiagnosticInfo;
                (data, val) = DiagnosticInfo::deserialize(data)?;
                value=vec![Value::DiagnosticInfo(val)];
            }
        }
        
        
        if encoding_mask & DataTypeId::ARRAY_DIM_ENCODED == DataTypeId::ARRAY_DIM_ENCODED
        {
            (data, array_dimension) = Vec::<i32>::deserialize(data)?;
        }
        
        Ok((
            data,
            Variant {
                value,
                array_dimension,
            },
        ))
    }
}


impl Variant {
    pub(crate) fn from<T:ToVariant>(value:T)->Variant {
        value.to_variant()
    }

    pub(crate) fn from_datatype(type_:u8,value:&str)->MapperResult<Variant>{
        match type_{
            DataTypeId::BOOLEAN=>{
                if value=="true"{
                    Ok(true.to_variant())
                }else{
                    Ok(false.to_variant())
                }
            },
            DataTypeId::SBYTE=>{
                let sbyte=value.parse::<i8>();
                match sbyte{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::BYTE=>{
                let byte=value.parse::<u8>();
                match byte{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::INT_16=>{
                let int16=value.parse::<i16>();
                match int16{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::UINT_16=>{
                let uint16=value.parse::<u16>();
                match uint16{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::INT_32=>{
                let int32=value.parse::<i32>();
                match int32{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::UINT_32=>{
                let uint32=value.parse::<u32>();
                match uint32{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::INT_64=>{
                let int64=value.parse::<i64>();
                match int64{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::UINT_64=>{
                let uint64=value.parse::<u64>();
                match uint64{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::FLOAT=>{
                let float=value.parse::<f32>();
                match float{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::DOUBLE=>{
                let double=value.parse::<f64>();
                match double{
                    Ok(value)=>Ok(value.to_variant()),
                    Err(_)=>Err(MapperError::new(MapperErrorKind::VariantError, "incompatible type for variant"))
                }
            },
            DataTypeId::STRING=>{
                Ok(value.to_variant())
            },
            _=> {
                Err(MapperError::new(MapperErrorKind::VariantError, "unkown type for variant"))
            },

        }
    }
}