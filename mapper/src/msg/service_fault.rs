use crate::uatypes::node_id::NodeId;
use crate::encoding_prelude::*;

use super::header::{prelude::*};


#[derive(Debug, Serialize)]
pub struct ServiceFault{
    pub(crate) message_header: MessageHeader,
    pub(crate) security_header: SecurityHeader,
    pub(crate) sequence_header: SequenceHeader,
    pub(crate) node_id: NodeId,
    pub(crate) response_header: ResponseHeader,
}

impl Deserialize for ServiceFault{
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> where Self: Sized {
        let (data_s,message_header)=MessageHeader::deserialize(data)?;
        let (data,security_header)=SymmetricSecurityHeader::deserialize(data_s)?;
        let (data,sequence_header)=SequenceHeader::deserialize(data)?;
        let node_id=NodeId::deserialize(data);
        match node_id{
            Ok((data,node))=>{
                let (data,response_header)=ResponseHeader::deserialize(data)?;
                Ok((data,ServiceFault{
                    message_header ,
                    security_header:SecurityHeader::Symmetric(security_header) ,
                    sequence_header ,
                    node_id:node,
                    response_header,
                }))
            },
            Err(_) =>{
                let (data,security_header)=AsymmetricSecurityHeader::deserialize(data_s)?;
                let (data,sequence_header)=SequenceHeader::deserialize(data)?;
                let (data,node_id)=NodeId::deserialize(data)?;
                let (data,response_header)=ResponseHeader::deserialize(data)?;
                Ok((data,ServiceFault{
                    message_header ,
                    security_header:SecurityHeader::Asymmetric(security_header) ,
                    sequence_header ,
                    node_id,
                    response_header,
                }))
            },
        }


        

    }
}