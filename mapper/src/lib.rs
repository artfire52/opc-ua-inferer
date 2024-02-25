#![allow(dead_code)]

//! # mapper
//!
//! `mapper` is a rust project to provide a python API of raw OPC-UA.
//! The aim is to use correspondance between abstract message such as
//! "open connection" to real message. Thus it can be used with Lstar
//! algorithm to detect vulnerabilities in OPC-UA implementation.

pub mod crypto;
pub mod handle;
pub mod msg;
pub mod uatypes;
mod error;

use error::{MapperError, MapperErrorKind};





pub(crate) type MapperResult<T>= Result<T,MapperError>;
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> where Self: Sized;

    fn take_count<'b>(data: &'b [u8], count: usize)-> MapperResult<(&[u8],Vec<u8>)>{
        let result:nom::IResult<& [u8], & [u8]> = nom::bytes::complete::take(count)(data);
        let output:MapperResult<(&[u8],Vec<u8>)> = match result {
            Ok((data, parsed)) => Ok((data, parsed.to_vec())),
            Err(_) => Err(MapperError::new(MapperErrorKind::ParsingError,"deserialize count failed")),
        };
        output
    }
}
#[cfg(feature="python")]
use crate::handle::python::Mapper;
#[cfg(feature="python")]
use cpython::py_module_initializer;
#[cfg(feature="python")]
use crate::uatypes::prelude::*;
#[cfg(feature="python")]
py_module_initializer!(mapper, |py, m| {
    m.add_class::<Mapper>(py)?;
    m.add(py,"__doc__","OPC UA mapper class")?;
    m.add(py,"bool",DataTypeId::BOOLEAN)?;
    m.add(py,"int8",DataTypeId::SBYTE)?;
    m.add(py,"uint8",DataTypeId::BYTE)?;
    m.add(py,"int16",DataTypeId::INT_16)?;
    m.add(py,"uint16",DataTypeId::UINT_16)?;
    m.add(py,"int32",DataTypeId::INT_32)?;
    m.add(py,"uint32",DataTypeId::UINT_32)?;
    m.add(py,"int64",DataTypeId::INT_64)?;
    m.add(py,"uint64",DataTypeId::UINT_64)?;
    m.add(py,"float",DataTypeId::FLOAT)?;
    m.add(py,"double",DataTypeId::DOUBLE)?;
    m.add(py,"NodeIdNumeric",crate::uatypes::node_id::EncodingValue::NUMERIC)?;
    m.add(py,"NodeIdString",crate::uatypes::node_id::EncodingValue::STRING)?;
    m.add(py,"NodeIdGuid",crate::uatypes::node_id::EncodingValue::GUID)?;
    m.add(py,"NodeIdByteString",crate::uatypes::node_id::EncodingValue::BYTE_STRING)?;
    Ok(())
});


pub mod encoding_prelude{
    pub(crate) use crate::Serialize;
    pub(crate) use crate::Deserialize;
    pub(crate) use crate::MapperResult;
    pub(crate) use derive_macro::{Deserialize, Serialize};
}

pub mod result_prelude{
    pub(crate) use crate::error::*;
    pub(crate) use crate::MapperResult;
}
