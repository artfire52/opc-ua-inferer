
use std::fmt::Debug;

use crate::uatypes::prelude::*;
use crate::msg::header::prelude::*;
use crate::encoding_prelude::*;

#[derive(Debug,Serialize,Deserialize)]
pub(crate) struct WriteRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) nodes_to_write: Vec::<WriteValue>,
}


impl WriteRequest {
    pub fn build(session_node_id:&NodeId,node_id_to_write:&NodeId,data_value:&DataValue)-> WriteRequest{
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
        let node_id: NodeId = NodeId::new_numeric(0, 673);
        let request_header = RequestHeader {
            authentication_token: session_node_id.clone(),
            timestamp: DateTime::new_now(),
            request_handle: 0,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let nodes_to_write= vec![WriteValue{
            node_id:node_id_to_write.clone(),
            attribute_id:AttributeId::VALUE,
            index_range:UaString::new(),
            value: data_value.clone()
        }];
        WriteRequest{
            message_header: message_header,
            security_header: security_header,
            sequence_header: sequence_header,
            node_id: node_id,
            request_header: request_header,
            nodes_to_write: nodes_to_write,
        }
    }
}



#[derive(Deserialize,Serialize,Debug)]
pub(crate) struct WriteResponse{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) results: Vec::<u32>,
    pub(crate) diagnostic_info: Vec::<DiagnosticInfo>,
}

