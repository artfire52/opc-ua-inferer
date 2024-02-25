//! Contain all possible message and headers.
//! It also contains functions to handle messages and their signatures.

use derive_macro::DebugOutputEnum;
use derive_macro::Serialize;

use crate::MapperResult;
use crate::crypto::encryption::EncryptionType;
use crate::crypto::security_policy::SecurityPolicy;
use crate::crypto::security_policy::SecurityPolicyUri;
use crate::uatypes::security_mode::MessageSecurityMode;
use crate::Deserialize;
use crate::Serialize;
use crate::result_prelude::*;

use prelude::*;

use self::write::WriteRequest;
use self::write::WriteResponse;

pub mod acknowledge;
pub mod close_secure_channel;
pub mod create_session;
pub mod active_sessions;
pub mod error;
pub mod get_endpoints;
pub mod header;
pub mod hello;
pub mod message_type;
pub mod open_secure_channel;
pub mod parser;
pub mod reverse_hello;
pub mod close_session;
pub mod service_fault;
pub mod read;
pub mod nullsize;
pub mod write;

pub mod prelude {
    pub(crate) use crate::msg::acknowledge::*;
    pub(crate) use crate::msg::active_sessions::*;
    pub(crate) use crate::msg::close_secure_channel::*;
    pub(crate) use crate::msg::create_session::*;
    pub(crate) use crate::msg::error::*;
    pub(crate) use crate::msg::message_type::*;
    pub(crate) use crate::msg::get_endpoints::*;
    pub(crate) use crate::msg::hello::*;
    pub(crate) use crate::msg::open_secure_channel::*;
    pub(crate) use crate::msg::reverse_hello::*;
    pub(crate) use crate::msg::service_fault::*;
    pub(crate) use crate::msg::close_session::*;
    pub(crate) use crate::msg::read::*;
    pub(crate) use crate::msg::nullsize::*;
    pub(crate) use crate::msg::write::*;
}

//set the size of the message with an serialize message.
// This function usefull only when the msg is not encrypted.
pub fn set_size(msg: &mut Vec<u8>) {
    let size = (msg.len() as u32).serialize();
    msg[4..8].copy_from_slice(&size);
}

//set the size with a custom value. It is very usefull for padding.
//This function does not required to be called, only padding function call it.
//This function may be public for debugging purporse.
pub(crate) fn set_size_custom(msg: &mut Vec<u8>, size: usize) {
    let size = (size as u32).serialize();
    msg[4..8].copy_from_slice(&size);
}


pub fn set_secure_channel_id(msg: &mut Vec<u8>, secure_channel_id: u32) {
    let secure_channel_id = secure_channel_id.serialize();
    msg[8..12].copy_from_slice(&secure_channel_id);
}
//Get the size of a serialized message.
// It is usefull for parsing purpose
pub fn get_size(msg: &[u8]) -> crate::MapperResult<u32> {
    let size = &msg[4..8];
    Ok(u32::deserialize(size)?.1)
}

pub fn set_secure_token_id(msg: &mut Vec<u8>, token_id: u32) {
    let token_id = token_id.serialize();
    msg[12..16].copy_from_slice(&token_id);
}

pub fn set_sequence_number(msg: &mut Vec<u8>, sequence_number: u32) {
    let sequence_number = sequence_number.serialize();
    let offset= get_offset_to_encrypt(msg).unwrap();
    msg[offset..offset+4].copy_from_slice(&sequence_number);
}

pub fn set_request_id(msg: &mut Vec<u8>, request_id: u32) {
    let token_id = request_id.serialize();
    let offset= get_offset_to_encrypt(msg).unwrap();
    msg[offset+4..offset+8].copy_from_slice(&token_id);
}

pub (crate) fn set_intermediate_chunk(msg: &mut Vec<u8>){
    msg[3]=67;//b"C"
}



// Extra padding required for keysize > 2048 bits (256 bytes)
// the minimum padding correspond to field "padding size" and eventually "extra padding size"
pub (crate) fn minimum_padding(key_length: usize) -> usize {
    if key_length <= 256 {
        1
    } else {
        2
    }
}





//The padding size is just the number of bytes to ensure that we encrypt an int value of block
pub(crate) fn padding_size(
    security_policy_uri: &str,
    security_mode: u32,
    key_length: usize,
    plain_text_block_size: usize,
    signature_size: usize,
    bytes_to_writes_encrypted: usize,
    is_asymmetric: bool,//if asymmetric padding is mandatory even for policy sign only
) -> (usize, usize) {
    if security_policy_uri != SecurityPolicyUri::None && (security_mode==MessageSecurityMode::SIGN_AND_ENCRYPT || (security_mode==MessageSecurityMode::SIGN && is_asymmetric) )
    {
        //PaddingSize = PlainTextBlockSize – ((BytesToWrite_encrypted + SignatureSize + 1) % PlainTextBlockSize);
        //minimum padding is the padding size and eventually the extra padding size. (one byte each)
        let minimum_padding = minimum_padding(key_length);
        let encrypt_size = bytes_to_writes_encrypted + signature_size + 1;
        let padding_size = if encrypt_size % plain_text_block_size != 0 {
            plain_text_block_size - (encrypt_size % plain_text_block_size)
        } else {
            0
        };
        (minimum_padding + padding_size, minimum_padding)
    } else {
        (0, 0)
    }
}


//Private function to handle unauthenticaed padding
//this fucntion require a full policy security do not call earlier
fn set_padding_signature_unauthenticated(
    security_mode: u32,
    security_policy: &mut SecurityPolicy,
    body: &mut Vec<u8>,
    is_asymmetric: bool,
) ->MapperResult<()>{
    let offset = get_offset_to_encrypt(body)?;
    let cipher_text_block_size: usize;
    let plain_text_block_size: usize;
    let signature_size: usize;

    if is_asymmetric {
        //verify that all required element are present
        let signer=match security_policy.asymmetric_signature_algorithm.as_ref(){
            Some(sign)=>sign,
            _ => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing asym keys"))
        };
       
        let encrypter_decrypter =match security_policy.asymmetric_encryption.as_ref(){
            Some(enc)=>enc,
            _ => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing asym keys"))
        }; 
        
        //for rsa cyphertext block size is the same as key length
        (plain_text_block_size, cipher_text_block_size) = (
            encrypter_decrypter.plain_text_block_size(body),
            encrypter_decrypter.encrypter.encrypt_len(body).unwrap(),
        );
        signature_size = signer.signature_size();
    }else{
        let signer = match security_policy.symmetric_signature_algorithm_client.as_ref(){
            Some(enc)=>enc,
            _ => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing sym key"))
        }; 
        let sym_cipher = match security_policy.symmetric_encryption_client.as_ref(){
            Some(enc)=>enc,
            _ => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing sym key"))
        }; 


        (plain_text_block_size, cipher_text_block_size) = sym_cipher.plain_text_block_size_key_size();
        signature_size = signer.signature_size();
    }
    let (padding_size, minimum_padding) = padding_size(
        security_policy.policy_uri,
        security_mode,
        cipher_text_block_size,
        plain_text_block_size,
        signature_size,
        body[offset..].len(),
        is_asymmetric 
    );
    if padding_size > 0 {
        // A number of bytes are written out equal to the padding size.
        // Each byte is the padding size. So if padding size is 15 then
        // there will be 15 bytes all with the value 15
        if minimum_padding == 1 {
            let padding_byte = ((padding_size - 1) & 0xff) as u8;
            let _ = write_bytes(body, padding_byte, padding_size);
        } else if minimum_padding == 2 {
            // Padding and then extra padding
            let padding_byte = ((padding_size - 2) & 0xff) as u8;
            let extra_padding_byte = ((padding_size - 2) >> 8) as u8;
            let _ = write_bytes(body, padding_byte, padding_size - 1);
            body.push(extra_padding_byte);
        }
        //total size of the packet. Offset is the size of plaintext headers (message header and security header)
        //the other part is the size of encrypted bytes. This field to be set before signing.
        let size:usize;
        if is_asymmetric {
            size = offset
                + ((body[offset..].len() + signature_size) / plain_text_block_size)
                    * cipher_text_block_size;
        }else{
            size = offset
                    + ((body[offset..].len() + signature_size) / plain_text_block_size)
                        * plain_text_block_size;
        }
        set_size_custom(body, size);
        let signature:Vec<u8>;

        if is_asymmetric {
            signature=security_policy.asymmetric_signature_algorithm.as_mut().unwrap().sign(body);
        }else{
            signature=security_policy.symmetric_signature_algorithm_client.as_mut().unwrap().sign(body);
        }

        body.extend_from_slice(&signature); 
    }else if padding_size==0 && security_mode==MessageSecurityMode::SIGN && !is_asymmetric{
        let size:usize;
        size=body.len() + signature_size;
        set_size_custom(body, size);
        let signature:Vec<u8>;

        signature=security_policy.symmetric_signature_algorithm_client.as_mut().unwrap().sign(body);
        body.extend_from_slice(&signature); 
    }
    Ok(())
}


//this function handle the padding and signature depending on message (asymmetric or symmetric to do)
//This function will also need to ginore padding for authenticated ecnryption (none implemented now)
pub(crate) fn set_padding_signature(
    security_mode: u32,
    security_policy: &mut SecurityPolicy,
    body: &mut Vec<u8>,
)->MapperResult<()> {
    set_size(body);
    if security_mode==MessageSecurityMode::NONE{
        return Ok(());
    }
    let message_type = MessageType::from(&body[..3]).unwrap();
    let is_asymmetric=match message_type{
        MessageType::OPN=> true,
        MessageType::HEL|MessageType::ACK|MessageType::RHE|MessageType::ERR=> {set_size(body);return Ok(())}
        _=> false,
    };
    set_padding_signature_unauthenticated(security_mode,security_policy,body,is_asymmetric)?;
    Ok(())
}

fn type_required_encryption(body: &[u8])->EncryptionType{
    let message_type = MessageType::from(&body[..3]).unwrap();
    match message_type{
        MessageType::OPN=> EncryptionType::Asymmetric,
        MessageType::HEL|MessageType::ACK|MessageType::RHE|MessageType::ERR=> EncryptionType::None,
        _=> EncryptionType::Symmetric,
    }
}


fn encrypt_msg_asymmetric(security_policy: &mut SecurityPolicy, body:&mut Vec<u8>,offset: usize)->MapperResult<()>{
    let encrypted=match &mut security_policy.asymmetric_encryption {
        Some(cipher) => cipher.encrypt(&body[offset..]),
        None => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing asym key")),
    };
    body.truncate(offset);
    body.extend_from_slice(&encrypted);
    Ok(())
}

fn encrypt_msg_symmetric_client(security_policy: &mut SecurityPolicy, body:&mut Vec<u8>,offset: usize)->MapperResult<()>{
    let encrypted=match &mut security_policy.symmetric_encryption_client {
        Some(cipher) => cipher.encrypt(&body[offset..]),
        None => return Err(MapperError::new(MapperErrorKind::MissingKey,"Missing sym key")),
    };
    body.truncate(offset);
    body.extend_from_slice(&encrypted);
    Ok(())
}



pub (crate) fn encrypt_msg(security_mode: u32,security_policy: &mut SecurityPolicy, body:&mut Vec<u8>)->MapperResult<()>{
    if security_mode==MessageSecurityMode::NONE{
        return Ok(());
    }
    let offset = get_offset_to_encrypt(body)?;
    match type_required_encryption(&body){
        EncryptionType::Asymmetric =>encrypt_msg_asymmetric(security_policy, body,offset)?,
        EncryptionType::Symmetric if security_mode==MessageSecurityMode::SIGN_AND_ENCRYPT =>encrypt_msg_symmetric_client(security_policy, body,offset)?,
        _=>{},
    }
    Ok(())

}

fn decrypt_msg_asymmetric(security_policy: &mut SecurityPolicy, body:&mut Vec<u8>,offset: usize){
    let decrypted=match &mut security_policy.asymmetric_encryption {
        Some(cipher) => cipher.decrypt(&body[offset..]),
        None => return,
    };
    body.truncate(offset);
    body.extend_from_slice(&decrypted);
}


pub fn decrypt_msg_symmetric_client(security_policy: &mut SecurityPolicy, body:&mut Vec<u8>,offset: usize){
    
    let decrypted=match &mut security_policy.symmetric_encryption_server {
        Some(cipher) => cipher.decrypt(&body[offset..]),
        None => return,
    };
    
    body.truncate(offset);
    body.extend(decrypted);
}



pub (crate) fn decrypt_msg(security_mode: u32,security_policy: &mut SecurityPolicy, body:&mut Vec<u8>)->MapperResult<()>{
    if security_mode==MessageSecurityMode::NONE{
        return Ok(());
    }
    let offset = get_offset_to_encrypt(body)?;
    match type_required_encryption(&body){
        EncryptionType::Asymmetric=>decrypt_msg_asymmetric(security_policy, body,offset),
        EncryptionType::Symmetric if security_mode==MessageSecurityMode::SIGN_AND_ENCRYPT=>decrypt_msg_symmetric_client(security_policy, body,offset),
        EncryptionType::None=>{},
        _=>{},
    }
    Ok(())

}


/// Writes a series of identical bytes to the stream
pub(crate) fn write_bytes(stream: &mut Vec<u8>, value: u8, count: usize) {
    stream.extend(vec![value; count])
}

///List of supported message.
#[derive(Debug,DebugOutputEnum,Serialize)]
pub (crate) enum Msg {
    HelloMessage(HelloMessage),
    AckowledgeMessage(AckowledgeMessage),
    OpenSecureChannelRequest(OpenSecureChannelRequest),
    OpenSecureChannelResponse(OpenSecureChannelResponse),
    GetEndPointsRequest(GetEndPointsRequest),
    GetEndPointsResponse(GetEndPointsResponse),
    CloseSecureChannelRequest(CloseSecureChannelRequest),
    CreateSessionRequest(CreateSessionRequest),
    RevHelloMessage(RevHelloMessage),
    ErrorMessage(ErrorMessage),
    CreateSessionResponse(CreateSessionResponse),
    CloseSessionRequest(CloseSessionRequest),
    CloseSessionResponse(CloseSessionResponse),
    ServiceFault(ServiceFault),
    ActiveSessionRequest(ActiveSessionRequest),
    ActiveSessionResponse(ActiveSessionResponse),
    ReadRequest(ReadRequest),
    ReadResponse(ReadResponse),
    NullSize(NullSize),
    WriteRequest(WriteRequest),
    WriteResponse(WriteResponse),
    
}


impl Msg {
    pub fn get_nonce(&self) -> &Vec<u8> {
        match self {
            Msg::OpenSecureChannelRequest(s) => &s.client_nonce.value.as_ref().unwrap(),
            Msg::OpenSecureChannelResponse(s) => &s.server_nonce.value.as_ref().unwrap(),
            _ => panic!("no nonce here"),
        }
    }
}



pub fn get_type(encoded_messages: &Vec<u8>) -> MessageType {
    MessageType::from(&encoded_messages[..3]).unwrap()
}



///this function return the the first bytes position that required encryption.
pub fn get_offset_to_encrypt(encoded_messages: &Vec<u8>) -> crate::MapperResult<usize> {
    let message_type = MessageType::from(&encoded_messages[..3])?;
    match message_type {
        MessageType::MSG => Ok(16),
        MessageType::OPN => {
            let mut offset = 12; //message header size
            for _i in 0..3{
                let add_offset=match i32::deserialize(&encoded_messages[offset..]){
                    Ok((_,-1))=>0,
                    Ok((_,int))=>int,
                    Err(_)=> 0,
                };
                offset = offset + add_offset as usize + 4; 
            }
             //loop i is equivalent but we cop with the case of an -1 meaning null Bytestring (only the size is encoded)   
            // offset = offset + i32::deserialize(&encoded_messages[offset..])?.1 as usize + 4; //4 is the size of the bytestring encoded legnth sec uri
            // offset = offset + i32::deserialize(&encoded_messages[offset..])?.1 as usize + 4; // sender certificate
            // offset += 24; //sha1 of cert (thumbprint) + bytestring encoding
            Ok(offset)
        }
        _ => Ok(16),//encryption not required
    }
}



