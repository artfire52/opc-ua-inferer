use crate::encoding_prelude::*;

use super::string::UaString;
use crate::MapperResult;

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct QualifiedName {
    pub(crate) namespace_index: u16,
    pub(crate) name: UaString,
}

impl QualifiedName {
    pub fn new(namespace_index: u16, name: UaString) -> QualifiedName {
        QualifiedName {
            namespace_index,
            name,
        }
    }
    pub fn empty() -> QualifiedName {
        QualifiedName {
            namespace_index:0,
            name:UaString::new(),
        }
    }
}
