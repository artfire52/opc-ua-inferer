use crate::{uatypes::prelude::*, {MapperError, MapperErrorKind}};
use crate::encoding_prelude::*;
use super::{prelude::*, Msg};
///This function parse message base on their type. For message MSG (regular message such as client request we need to parse the NodeId)
/// This function is usefull when the message is not encrypted. The type could be determine with operation on message header (never encrypted).
pub (crate) fn parse(data: &[u8]) -> MapperResult<Msg> {
    let message_type = MessageType::from(&data[..3])?;
    let msg=match message_type {
        MessageType::HEL => Msg::HelloMessage(HelloMessage::deserialize(data)?.1),
        MessageType::ACK => Msg::AckowledgeMessage(AckowledgeMessage::deserialize(data)?.1),
        MessageType::RHE => Msg::RevHelloMessage(RevHelloMessage::deserialize(data)?.1),
        MessageType::ERR => Msg::ErrorMessage(ErrorMessage::deserialize(data)?.1),
        //for other kind of message we need to know thanks to the nodeId,
        _ => parse_node_id(data)?,
    };
    Ok(msg)
}

//get the id number from node id. It is usefull only for MSG, CLO and OPN because
//others message are identify by their type
fn get_node_id(data: &[u8])->MapperResult<u32>{
    let message_type = MessageType::from(&data[..3])?;
    let node_id=match message_type {
        //we take the node_id after the message header + security header (symmetric)+ sequence header (12+4+8=24)
        MessageType::MSG => NodeId::deserialize(&data[24..])?.1,
        MessageType::CLO => NodeId::deserialize(&data[24..])?.1,
        MessageType::OPN => {
                let mut offset = 12; //message header size
                for _i in 0..3{
                    let add_offset=match i32::deserialize(&data[offset..]){
                        Ok((_,-1))=>0,
                        Ok((_,int))=>int,
                        Err(_)=> 0,
                    };
                    offset = offset + add_offset as usize + 4; 
                }
                offset +=8; //sequence header size
                NodeId::deserialize(&data[offset..])?.1
            },
        _ => return Err(MapperError::new(MapperErrorKind::ParsingError,"Unknown_node_id: deserialisation failed")),
    };
    if let Identifier::Numeric(id) = node_id.identifier{
        Ok(id)
    }
    else{
        Err(MapperError::new(MapperErrorKind::ParsingError,"Unknown_node_id: wrong node id format"))
    }
}

//Get the right message deserialization from the node_id
//This will evolved with message implemented
fn parse_node_id(data: &[u8]) -> MapperResult<Msg> {
    let id =get_node_id(data)?;
    let msg=match id {
        446 => Msg::OpenSecureChannelRequest(OpenSecureChannelRequest::deserialize(data)?.1),
        449 => Msg::OpenSecureChannelResponse(OpenSecureChannelResponse::deserialize(data)?.1),
        428 => Msg::GetEndPointsRequest(GetEndPointsRequest::deserialize(data)?.1),
        431 => Msg::GetEndPointsResponse(GetEndPointsResponse::deserialize(data)?.1),
        461 => Msg::CreateSessionRequest(CreateSessionRequest::deserialize(data)?.1),
        464 => Msg::CreateSessionResponse(CreateSessionResponse::deserialize(data)?.1),
        473 => Msg::CloseSessionRequest(CloseSessionRequest::deserialize(data)?.1),
        476 => Msg::CloseSessionResponse(CloseSessionResponse::deserialize(data)?.1),
        397 => Msg::ServiceFault(ServiceFault::deserialize(data)?.1),
        467 => Msg::ActiveSessionRequest(ActiveSessionRequest::deserialize(data)?.1),
        470 => Msg::ActiveSessionResponse(ActiveSessionResponse::deserialize(data)?.1),
        452 => Msg::CloseSecureChannelRequest(CloseSecureChannelRequest::deserialize(data)?.1),
        631 => Msg::ReadRequest(ReadRequest::deserialize(data)?.1),
        634 => Msg::ReadResponse(ReadResponse::deserialize(data)?.1),
        673 => Msg::WriteRequest(WriteRequest::deserialize(data)?.1),
        676 => Msg::WriteResponse(WriteResponse::deserialize(data)?.1),
        _ => return Err(MapperError::new(MapperErrorKind::ParsingError,"bad unknown type or not implemented yet")),
    };
    Ok(msg)
}