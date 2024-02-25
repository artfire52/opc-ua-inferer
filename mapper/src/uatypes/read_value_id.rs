use derive_macro::{Serialize, Deserialize};

// use super::qualified_name::QualifiedName;
use super::node_id::*;
use super::string::*;
use super::qualified_name::*;
use crate::encoding_prelude::*;


#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct ReadValueId {
    pub(crate) node_id:NodeId,
    pub(crate) attribute_id: u32,
    pub(crate) index_range: UaString, //null for non vector data or to get all vector
	pub(crate) data_encoding: QualifiedName,// not used (i.e. always use the default encoding of the session)

}



