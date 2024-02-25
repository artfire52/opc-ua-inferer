use crate::encoding_prelude::*;

use crate::{
    uatypes::{
        date_time::DateTime, endpoint_description::EndpointDescription, node_id::NodeId,
        string::UaString,
    }
};

use super::{
    header::{
        message_header::MessageHeader, request_header::RequestHeader,
        response_header::ResponseHeader, security_header::SymmetricSecurityHeader,
        sequence_header::SequenceHeader,
    },
    message_type::MessageType,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEndPointsRequest {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) request_header: RequestHeader,
    pub(crate) endpoint_url: UaString,
    pub(crate) local_id: Vec::<UaString>,
    pub(crate) profile_uris: Vec::<UaString>,
}

impl GetEndPointsRequest {
    pub fn build(secure_channel_id: u32, token_id: u32) -> GetEndPointsRequest {
        let mut message_header: MessageHeader = Default::default();
        message_header.secure_channel_id = secure_channel_id;
        message_header.message_type = MessageType::MSG;
        let mut security_header: SymmetricSecurityHeader = Default::default();
        security_header.token_id = token_id;
        let sequence_header: SequenceHeader = SequenceHeader {
            sequence_number: 2,
            request_id: 2,
        };
        let node_id: NodeId = NodeId::new_numeric(0, 428);
        let request_header = RequestHeader {
            authentication_token: NodeId::new_numeric(0, 0),
            timestamp: DateTime::new_now(),
            request_handle: 1,
            return_diagnostic: 0,
            audit_entry: UaString::new(),
            timout_hint: 0,
            additional_header: None,
        };
        let endpoint_url = UaString::from("opc.tcp://localhost:4840");
        let local_id = Vec::new();
        let profile_uris = Vec::new();
        GetEndPointsRequest {
            message_header,
            security_header,
            sequence_header,
            node_id,
            request_header,
            endpoint_url,
            local_id,
            profile_uris,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetEndPointsResponse {
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SymmetricSecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
    pub(crate) endpoints: Vec::<EndpointDescription>,
}
