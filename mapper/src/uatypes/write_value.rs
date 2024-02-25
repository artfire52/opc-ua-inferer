use crate::encoding_prelude::*;
use super::{node_id::*, string::UaString, data_value::DataValue,attribute_id::*};
#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct WriteValue {
    pub(crate) node_id:NodeId,
    pub(crate) attribute_id:u32,
    pub(crate) index_range:UaString,
    pub(crate) value:DataValue,
}

impl WriteValue{
    pub (crate) fn build(node_id: &NodeId, value:&DataValue)->Self{
        WriteValue { node_id: node_id.clone(),
                     attribute_id: AttributeId::VALUE,
                     index_range: UaString::new(),
                     value: value.clone() }
    }
}