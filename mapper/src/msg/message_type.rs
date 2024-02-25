use crate::{encoding_prelude::*, error::MapperError, error::MapperErrorKind};

#[derive(Debug)]
pub enum MessageType {
    HEL,
    ACK,
    ERR,
    RHE,
    MSG,
    OPN,
    CLO,
}

impl MessageType {
    pub fn value(&self) -> &[u8; 3] {
        match *self {
            MessageType::HEL => b"HEL",
            MessageType::ACK => b"ACK",
            MessageType::RHE => b"RHE",
            MessageType::ERR => b"ERR",
            MessageType::MSG => b"MSG",
            MessageType::OPN => b"OPN",
            MessageType::CLO => b"CLO",
            // _ =>b"ERR",
        }
    }
    pub fn from(value: &[u8]) -> MapperResult<MessageType> {
        match value {
            b"HEL" => Ok(MessageType::HEL),
            b"ACK" => Ok(MessageType::ACK),
            b"RHE" => Ok(MessageType::RHE),
            b"ERR" => Ok(MessageType::ERR),
            b"MSG" => Ok(MessageType::MSG),
            b"CLO" => Ok(MessageType::CLO),
            b"OPN" => Ok(MessageType::OPN),
            _ => Err(MapperError::new(MapperErrorKind::ParsingError, "parsing message type failed")),
        }
    }
}

impl Serialize for MessageType {
    fn serialize(&self) -> Vec<u8> {
        self.value().to_vec()
    }
}
impl Deserialize for MessageType {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        let (data, message_type) = MessageType::take_count(data, 3)?;
        let message_type = MessageType::from(&message_type)?;
        Ok((data, message_type))
    }
}
