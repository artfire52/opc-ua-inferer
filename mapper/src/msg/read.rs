
use std::fmt::Debug;

use crate::uatypes::prelude::*;
use crate::msg::header::prelude::*;
use crate::encoding_prelude::*;

#[derive(Debug,Serialize,Deserialize)]
pub(crate) struct ReadRequest{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) max_age:f64,
    pub(crate) timestamp_to_return: u32,
    pub(crate) nodes_to_read: Vec::<ReadValueId>,
}



pub struct TimeStampToReturn;
impl TimeStampToReturn{
    pub(crate) const SOURCE:u32 =0;
	pub(crate) const SERVER:u32 =1;
	pub(crate) const BOTH:u32 =2;
	pub(crate) const NEITHER:u32 =3;
	pub(crate) const INVALID:u32 =4;
}

impl ReadRequest{
    pub fn build(session_node_id:&NodeId,node_id_toread:&NodeId)-> ReadRequest{
        let message_header = MessageHeader {
            message_type: super::message_type::MessageType::MSG,
            is_final: b'F',
            message_size: 0,
            secure_channel_id: 0,
        };
        let security_header: SymmetricSecurityHeader = SymmetricSecurityHeader::default();

        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 2,
            request_id: 2,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 631);
        let request_header = RequestHeader {
            authentication_token: session_node_id.clone(),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let max_age=0.0 as f64;
        let timestamp_to_return=TimeStampToReturn::NEITHER;
        let nodes_to_read= vec![ReadValueId{
            node_id:node_id_toread.clone(),
            attribute_id:AttributeId::VALUE,
            index_range:UaString::new(),
            data_encoding: QualifiedName::empty(),
        }];
        ReadRequest{
            message_header: message_header,
            security_header: security_header,
            sequence_header: sequence_header,
            node_id: node_id,
            request_header: request_header,
            max_age:max_age,//either u32 or f64 it depends because duration can be both. Most likely to be f64 since u32 seems to be an exception for opensecurechannel message.
            timestamp_to_return: timestamp_to_return,
            nodes_to_read: nodes_to_read,
        }
    }
}



#[derive(Deserialize,Serialize,Debug)]
pub(crate) struct ReadResponse{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) result: Vec::<DataValue>,
    pub(crate) diagnostic_info: Vec::<DiagnosticInfo>,
}

