use crate::crypto::{hash};
use crate::crypto::pkey::{PrivateKey, PublicKey};
use crate::crypto::security_policy::SecurityPolicy;
use crate::crypto::security_policy::SecurityPolicyUri;
use crate::msg::close_secure_channel::CloseSecureChannelRequest;
use crate::msg::close_session::CloseSessionRequest;
use crate::msg::create_session::CreateSessionRequest;
use crate::msg::get_endpoints::{GetEndPointsRequest};
use crate::msg::prelude::*;
use crate::msg::message_type::MessageType;
use crate::msg::open_secure_channel::{OpenSecureChannelRequest};
use crate::msg::{self, Msg};
use crate::uatypes::prelude::*;

use crate::{Serialize};
use openssl::rsa::Rsa;
use openssl::x509::X509;
use core::panic;

use std::io::{prelude::*, ErrorKind};
// use crate::result_prelude::*;
use std::net::{TcpStream};
use std::{thread, time};

/// This struct is the one responsible for handling the connection data, generate data and deserialize data.
/// This struct might change according to new work (introduction of new message handling).
/// for now it is still a mess due to debug purpose

#[derive(Debug)]
enum OpcUaResponse{
    Eof,
    Some(Vec<u8>),
    NoResp,
}

pub struct Handle<'a> {
    //buffer information
    receiver_buffer_size: u32,
    send_buffer_size: u32,
    max_message_size: u32,
    max_chunk_out: u32,
    //for sequence header
    sequence_number: u32,
    request_id: u32,
    request_handle: u32,
    //for encryption purpose. Also required to establish secure connection.
    security_policy_uri: &'a str,
    security_mode: u32,
    security_false: bool,
    security_mode_save: u32,
    secure_channel_id: u32,
    token_id: u32,
    //keys
    server_public_key: Vec<PublicKey>,
    private_key: PrivateKey,
    private_key_false: PrivateKey,
    server_certificate: Vec<ByteString>,
    sender_certificate: ByteString,
    sender_false_certificate: ByteString,
    user_certificate:ByteString,
    user_false_certificate:ByteString,
    receiver_certificate_thumbprint: Vec<ByteString>,
    client_nonce:ByteString,
    server_nonce:ByteString,
    //connection purpose
    authentication_token:NodeId,
    security_policy_uri_token_anon: &'a str,
    security_policy_uri_token_user: &'a str,
    security_policy_uri_token_cert: &'a str,
    policy_id_anon: UaString,
    policy_id_user: UaString,
    policy_id_cert: UaString,
    shift_secure_token_id: bool,
    endpoint_url:Vec<UaString>,
    target_node:NodeId,
    target_node_value:DataValue,
    c_chunk:bool,


    
}

impl<'a> Handle<'a> {
        //define abstract messages
        pub const HELLO: &'static str = "hello";
        pub const OPN_REQ: &'static str = "open_secure_channel_request";
        pub const OPN_REQ_WRONG: &'static str = "open_secure_channel_request_wrong";
        pub const GET_ENDPOINT_REQ: &'static str = "get_endpoint_request";
        pub const CLO_REQ: &'static str = "close_secure_channel_request";
        pub const CREATE_SESS: &'static str = "create_session";
        pub const CREATE_SESS_SEC_TOKEN_ID: &'static str = "create_session_false_token_id";
        pub const CLOSE_SESS: &'static str = "close_session";
        pub const ACTIVE_SESS: &'static str = "active_session";
        pub const ACTIVE_SESS_SEC_TOKEN_ID: &'static str = "active_session_false_token_id";
        pub const SET_SEC_MODE_NONE: &'static str = "set_security_mode_none";
        pub const ACTIVE_SESS_ANON: &'static str = "active_session_anon";
        pub const ACTIVE_SESS_WRONG_USER: &'static str = "active_session_wrong_user";
        pub const ACTIVE_SESS_CERT: &'static str = "active_session_cert";
        pub const ACTIVE_SESS_WRONG_CERT: &'static str = "active_session_cert_wrong";
        pub const READ_REQ: &'static str = "read_req";
        pub const WRITE_REQ: &'static str = "write_req";
        pub const NULL_SIZE: &'static str = "nullsize";
        pub const OPEN_REQ_C_CHUNK:&'static str="open_secure_channel_c_chunk";



        //Define return value here as R_NAME
        pub const R_INTERNAL_UPDATE: &'static str = "internal_update,";
        pub const R_SERVICE_FAULT: &'static str = "Service_fault,";
        pub const R_ERR: &'static str = "Err,";
        pub const R_ACK: &'static str = "Ack,";
        pub const R_HELLO: &'static str = "Hello,";
        pub const R_OPEN_REQ: &'static str = "OpnReq,";
        pub const R_OPEN_REPOK: &'static str = "OpnRepOK,";
        pub const R_OPEN_REPNOK: &'static str = "OpnRepNOK,";
        pub const R_GEP_REQ: &'static str = "GepReq,";//GEP= get endpoints
        pub const R_GEP_REPNOK: &'static str = "GepResNOK,";
        pub const R_GEP_REPOK: &'static str = "GepResOK,";
        pub const R_CLOSE: &'static str = "Clo,";
        pub const R_CREATE_SESS_REQ: &'static str = "CreSesReq,";
        pub const R_CREATE_SESS_REPOK: &'static str = "CreSesResOK,";
        pub const R_CREATE_SESS_REPNOK: &'static str = "CreSesResNOK,";
        pub const R_REV_HELLO: &'static str = "RevHel,";
        pub const R_CLO_SESS_REQ: &'static str = "CloSesReq,";
        pub const R_CLO_SESS_REPOK: &'static str = "CloSesResOK,";
        pub const R_CLO_SESS_REPNOK: &'static str = "CloSesResNOK,";
        pub const R_ACTIVE_SESS_REQ: &'static str = "AcSesReq,";
        pub const R_ACTIVE_SESS_REPOK: &'static str = "AcSesResOK,";
        pub const R_ACTIVE_SESS_REPNOK: &'static str = "AcSesResNOK,";
        pub const R_READ_REQ: &'static str = "ReadReq,";
        pub const R_READ_REPNOK: &'static str = "ReadRepNOK,";
        pub const R_READ_REPOK: &'static str = "ReadRepOK,";
        pub const R_WRITE_REQ: &'static str = "WriteReq,";
        pub const R_WRITE_REPOK: &'static str = "WriteRepOK,";
        pub const R_WRITE_REPNOK: &'static str = "WriteRepNOK,";
        pub const R_PARSE_ERR: &'static str = "ParseErr,";
        pub const R_NULL_SIZE: &'static str = "nullsize,";
        pub const R_EOF: &'static str = "Eof,";
        pub const R_NO_RESP: &'static str = "No resp,";

    pub fn new_basic256_sha256(key_path:&str,false_key_path:&str,own_cert_path: &str,sender_false_certificate_path:&str,usr_cert_path:&str,usr_false_cert_path:&str,security_mode:u32) -> Handle<'a> {
        let sender_certificate = std::fs::read(own_cert_path).unwrap();

        let private_key_raw = std::fs::read(key_path).unwrap();
        let rsa = Rsa::private_key_from_der(&private_key_raw).unwrap();
        let private_key = PrivateKey::from_rsa(rsa).unwrap();
        let private_key_false_raw = std::fs::read(false_key_path).unwrap();
        let rsa = Rsa::private_key_from_der(&private_key_false_raw).unwrap();
        let private_key_false = PrivateKey::from_rsa(rsa).unwrap();
        let user_certificate_raw = std::fs::read(usr_cert_path).unwrap();
        let user_false_certificate_raw = std::fs::read(usr_false_cert_path).unwrap();
        let sender_false_certificate_raw = std::fs::read(sender_false_certificate_path).unwrap();

        let handle = Handle {
            receiver_buffer_size: 0,
            send_buffer_size: 0,
            max_message_size: 0,
            max_chunk_out: 0,
            sequence_number: 0,
            request_id: 0,
            request_handle: 0,
            security_policy_uri: SecurityPolicyUri::Basic256Sha256,
            security_mode:security_mode, //MessageSecurityMode::SIGN_AND_ENCRYPT,
            security_false: false,
            security_mode_save:security_mode, 
            secure_channel_id: 0,
            token_id: 0,
            server_public_key: vec![],
            private_key,
            private_key_false,
            server_certificate: vec![],//ByteString::from(server_cert_raw),
            sender_certificate: ByteString::from(sender_certificate),
            sender_false_certificate: ByteString::from(sender_false_certificate_raw),
            user_certificate:ByteString::from(user_certificate_raw),
            user_false_certificate:ByteString::from(user_false_certificate_raw),
            receiver_certificate_thumbprint: vec![],//ByteString::from(receiver_certificate_thumbprint),
            client_nonce:ByteString::new(),
            server_nonce:ByteString::new(),
            authentication_token:NodeId::new_numeric(0, 0),
            security_policy_uri_token_anon:SecurityPolicyUri::None,
            security_policy_uri_token_user:SecurityPolicyUri::None,
            security_policy_uri_token_cert:SecurityPolicyUri::None,
            policy_id_anon: UaString::new(),
            policy_id_user: UaString::new(),
            policy_id_cert: UaString::new(),
            shift_secure_token_id:false,
            endpoint_url:vec![],
            target_node:NodeId::empty(),
            target_node_value:DataValue::empty(),
            c_chunk:false,
        };
        handle
    }

    pub (crate) fn translate_from_object_to_binary(messages: Vec<Msg>) -> Vec<Vec<u8>> {
        messages.iter().map(|msg| msg.serialize()).collect()
    }

    pub fn set_target_node(&mut self,type_:u8,namespace:u16,id:&str){
        let identifier=NodeId::from_str_to_id(id, type_);
        self.target_node=NodeId::new(namespace, identifier);

    }

    pub fn set_target_node_value(&mut self,type_:u8,value:&str){
        self.target_node_value=DataValue::from_value(type_,value).unwrap();
    }

    pub fn get_server_certificate(&mut self,socket_addr:String,timeout :u64){
        let mut cert=None;
        let messages = vec![Handle::HELLO,Handle::OPN_REQ,Handle::GET_ENDPOINT_REQ,Handle::CLO_REQ];
        let messages_len=messages.len();
        let sleep_duration = time::Duration::from_millis(500);
        let mut stream;
        let mut i=0;
        loop{
            if let Ok(s) = TcpStream::connect(&socket_addr){
                stream=s;
                break;
            }
            if i==9{
                panic!("failed to connect to server");
            }
            i+=1;
            thread::sleep(sleep_duration);
           
        }
        stream.set_nonblocking(false).expect("set_nonblocking call failed");
        stream.set_read_timeout(Some(std::time::Duration::from_millis(timeout))).unwrap();
        let mut security_policy: SecurityPolicy;
        security_policy = SecurityPolicy::new(SecurityPolicyUri::None);
        self.security_mode=MessageSecurityMode::NONE;
        let channel_timeout:u32= (timeout as u32 *messages_len as u32) +2000;//handle channel timeout to avoid toomanysecurechannel error

        for (cpt,msg) in messages.into_iter().enumerate() {
            //session timeout is not usefull
            let msg = match self.translate_from_abstract_to_object(&UaString::from("opc.tcp://localhost:4840"),msg,&mut security_policy,&channel_timeout,&2000.0,&ByteString::new(),None,None){
                Some(m)=>m,
                _=> panic!("Unreachable"),
            };

            let _ = self.send_opcua(msg, &mut security_policy, &mut stream);
                let buffer = Handle::recv_opcua_response(&mut stream, 8,Some(1)); //function to set/change the secure policy
                for  i in buffer
                {   
                    match i{
                        OpcUaResponse::Eof=>{
                            if cpt!=messages_len-1 {
                                panic!("Could not get server certificate");
                            }
                        },
                        OpcUaResponse::NoResp=>{
                            if cpt!=messages_len-1 {
                                panic!("Could not get server certificate, no resp");
                            }
                            
                        },
                        OpcUaResponse::Some(buf)=>{
                            let msg = crate::msg::parser::parse(&buf);
                            match msg{
                                Ok(Msg::GetEndPointsResponse(m))=>{
                                    cert=Some(m.endpoints[0].server_certificate.clone());
                                    self.endpoint_url.push(m.endpoints[0].endpoint_url.clone());
                                    self.update_from_msg(&Msg::GetEndPointsResponse(m),&mut security_policy);
                                },
                                Ok(m)=>{
                                    self.update_from_msg(&m,&mut security_policy);
                                },
                                Err(_)=>{ 
                                    panic!("Could not get server certificat0e");
                                },
                            };
                        },
                    };
                }
            }
        
        let _ =stream.shutdown(std::net::Shutdown::Both);
        self.restore_state();
        let close_sec_sleep = time::Duration::from_millis(1);
        thread::sleep(close_sec_sleep);
        self.sequence_number=0;
        match cert{
            None=>panic!("impossible to have certificate of the target {}",socket_addr),
            Some(certificate)=>{
                if let None= certificate.value{
                    panic!("impossible to have certificate of the target {}",socket_addr);
                }
                let server_cert  = X509::from_der(certificate.value.as_deref().unwrap()).unwrap();
                self.receiver_certificate_thumbprint.push(ByteString::from(hash::compute_certificate_thumbprint(&server_cert)));
                self.server_certificate.push(certificate);
                self.server_public_key.push(server_cert.public_key().unwrap());

            },
        }
        println!("{}'s certificate received",socket_addr);
        
    }

    pub fn submit_word(&mut self,socket_addr:String, messages: Vec<&str>,target_index:usize,timeout :u64,nb_messages:Vec<usize>,known_no_resp:Vec<usize>) -> Vec<String> {
        let mut result=Vec::with_capacity(messages.len());
        let sleep_duration = time::Duration::from_millis(500);
        let mut stream;
        let mut i=0;
        loop{
            if let Ok(s) = TcpStream::connect(&socket_addr){
                stream=s;
                break;
            }
            if i==9{
                panic!("failed to connect to server");
            }
            i+=1;
            thread::sleep(sleep_duration);
           
        }
        stream.set_nonblocking(false).expect("set_nonblocking call failed");
        stream.set_read_timeout(Some(std::time::Duration::from_millis(timeout))).unwrap();
        let server_public_key: PublicKey =self.server_public_key[target_index].clone();
        let server_certificate: ByteString =self.server_certificate[target_index].clone();
        let receiver_certificate_thumbprint: ByteString = self.receiver_certificate_thumbprint[target_index].clone();
        let private_key: PrivateKey=self.private_key.clone();
        let private_key_false: PrivateKey=self.private_key_false.clone();
        let endpoint_url: UaString=self.endpoint_url[target_index].clone();
        let mut security_policy: SecurityPolicy;
        // self.security_mode=1;
        if self.security_mode == MessageSecurityMode::NONE{
            security_policy = SecurityPolicy::new(SecurityPolicyUri::None);
        }else{
            security_policy = SecurityPolicy::new(SecurityPolicyUri::Basic256Sha256);
        }

        //If the security policy is not none, asymmetric are required
        if self.security_policy_uri != SecurityPolicyUri::None {
            security_policy.set_asym(&private_key, &server_public_key);
        }
        
        let messages_len=messages.len();
        let session_timeout:f64= (timeout*messages_len as u64) as f64 +2000.0;//handle session timeout to avoid toomanysession error
        let channel_timeout:u32= (timeout as u32 *messages_len as u32) +2000;//handle channel timeout to avoid toosecurechannel error
      
        let mut sleep=false;//when the server has to deal with a closing secure channel message, it might require more time to finish.
        //This sleep is here to avoid non deterministic behaviour (to be sure that the server is available)
        let mut security_policy_changed:bool=false;
        for (cpt,msg) in messages.into_iter().enumerate() {
            if sleep{
                let sleeping_duration = time::Duration::from_millis(1);

                thread::sleep(sleeping_duration);
                sleep=false;
            }
            if msg==Handle::CLO_REQ{
                sleep=true;
            }
            let msg = match self.translate_from_abstract_to_object(&endpoint_url,msg,&mut security_policy,&channel_timeout,&session_timeout,&receiver_certificate_thumbprint,Some(&server_public_key),Some(&server_certificate)){
                Some(m)=>m,
                _=> {result.push(Handle::R_INTERNAL_UPDATE.to_string());
                    continue},
            };
            if self.security_false && !security_policy_changed && self.security_policy_uri != SecurityPolicyUri::None{
                // security_policy=security_policy_false;
                security_policy.set_asym(&private_key_false, &server_public_key);
                security_policy_changed=true;
            }
            let ret : std::io::Result<usize>= self.send_opcua(msg, &mut security_policy, &mut stream);
            match ret{
                Err(_)=> {
                    let len=messages_len-result.len();
                    for _ in 0..len{
                        result.push("Eof".to_string());
                    }
                    let _ =stream.shutdown(std::net::Shutdown::Both);
                    self.restore_state();
                    return result;
                },
                Ok(_)=>{}
            };

            let mut to_push:String;
            if !known_no_resp.contains(&cpt){
                let nb_msg= match nb_messages.get(cpt){
                    Some(nb)=>Some(*nb),
                    None=>None,
                };
                let buffer = Handle::recv_opcua_response(&mut stream, 8,nb_msg); //function to set/change the secure policy
                to_push=String::new();
                for  i in buffer
                {   
                    match i{
                        OpcUaResponse::Eof=>{
                            to_push.push_str(&Handle::R_EOF.to_string());
                            result.push(to_push);
                            let len=messages_len-result.len();
                            for _ in 0..len{
                                result.push(Handle::R_EOF.to_string());
                            }
                            let _ =stream.shutdown(std::net::Shutdown::Both);
                            self.restore_state();
                            return result;
                        },
                        OpcUaResponse::NoResp=>{
                            if to_push.len()==0{
                                to_push.push_str(&Handle::R_NO_RESP.to_string());
                            }
                        },
                        OpcUaResponse::Some(mut buf)=>{
                            let _=msg::decrypt_msg(self.security_mode,&mut security_policy, &mut buf);
                            let msg = crate::msg::parser::parse(&buf);
                            match msg{
                                Ok(m)=>{
                                    self.update_from_msg(&m,&mut security_policy);
                                    to_push.push_str(&Handle::update_response(&m));
                                    
                                },
                                Err(_)=>{ 
                                    to_push.push_str(&Handle::R_PARSE_ERR.to_string());
                                },
                            };
                        },
                    };
                }
            }else{
                to_push=String::from(Handle::R_NO_RESP);
            }
            result.push(to_push);
        }
        let _ =stream.shutdown(std::net::Shutdown::Both);
        self.restore_state();
        result
    
    }

    fn restore_state(&mut self){
        self.security_mode=self.security_mode_save;
        self.security_false=false;
    }
    ///when the amount of message is know we can use nb_message to receive precisely nb_messages
    fn recv_opcua_response(stream: &mut TcpStream, buffer_size: usize,nb_message:Option<usize>) -> Vec<OpcUaResponse> {
        let mut cpt:isize=0; 
        let nb_message=match nb_message{
            Some(n)=>n as isize,
            None=>-1,
        };
        let mut result:Vec<OpcUaResponse>=vec![];
        loop  {
            if cpt==nb_message{
                break;
            }
            let buffer_size = buffer_size;
            let mut buffer = vec![0; buffer_size];
            let recv_size=stream.read_exact(&mut buffer);
            match recv_size{
                Ok(_)=>{

                },
                Err(e) if e.kind()==ErrorKind::UnexpectedEof=> {
                    result.push(OpcUaResponse::Eof);
                    break;
                },
                Err(_)=> {
                    result.push(OpcUaResponse::NoResp);
                    break;
                },
            }

            let size = crate::msg::get_size(&buffer).unwrap();
            if size > buffer_size as u32 {
                let mut msg = buffer.to_vec();
                let mut buffer = vec![0; size as usize - buffer_size];
                stream.read_exact(&mut buffer).unwrap();
                msg.extend_from_slice(&buffer);
                result.push(OpcUaResponse::Some(msg));
            } else {
                result.push(OpcUaResponse::Some(buffer));
            }
            cpt+=1;
        }
        
        result
    }

    fn send_opcua(&mut self,msg: Msg, security_policy: &mut SecurityPolicy, stream: &mut TcpStream) -> std::io::Result<usize> {
        self.update_internal(&msg);
        let mut buffer_to_send=msg.serialize();
        //update present
        if let Msg::NullSize(_)=msg{
            self.pre_send_update(&mut buffer_to_send);
            let _=msg::set_padding_signature(self.security_mode,security_policy,&mut buffer_to_send);
            let _=msg::encrypt_msg(self.security_mode,security_policy, &mut buffer_to_send);
            msg::set_size_custom(&mut buffer_to_send, 0 as usize);
            stream.write(&buffer_to_send)
        }else{
            self.pre_send_update(&mut buffer_to_send);
            let _=msg::set_padding_signature(self.security_mode,security_policy,&mut buffer_to_send);
            let _=msg::encrypt_msg(self.security_mode,security_policy, &mut buffer_to_send);
            stream.write(&buffer_to_send)
        }
    }

    pub (crate) fn update_from_msg(&mut self, msg: &Msg,security_policy:&mut  SecurityPolicy) {
        match msg {
            Msg::HelloMessage(m)=>{
                self.receiver_buffer_size=m.receiver_buffer_size;
                self.send_buffer_size=m.send_buffer_size;
                self.max_message_size=m.max_msg_size;
                self.max_chunk_out=m.max_chunk_count;

            },
            Msg::AckowledgeMessage(m)=>{
                self.receiver_buffer_size=m.receiver_buffer_size;
                self.send_buffer_size=m.send_buffer_size;
                self.max_message_size=m.max_msg_size;
                self.max_chunk_out=m.max_chunk_count;

            },
            Msg::OpenSecureChannelResponse(m) => {
                self.secure_channel_id = m.secure_channel_id;
                self.token_id = m.token_id;
                if self.security_mode!=MessageSecurityMode::NONE{
                    self.server_nonce= m.server_nonce.clone();
                    
                    //derive key
                    security_policy.derive_symmetric_client(&self.client_nonce, &self.server_nonce).expect("haaaaaaaa");
                }

            },
            Msg::OpenSecureChannelRequest(m) => {
                //derive key
                self.client_nonce= m.client_nonce.clone();

            }
            Msg::CreateSessionResponse(m) => {
                //derive key
                self.server_nonce= m.server_nonce.clone();
                self.authentication_token= m.authentication_token.clone();
                let endpoint_array=m.endpoints.clone();
                for i in endpoint_array{
                    //we check if we have security policies
                    if security_policy.policy_uri!=i.security_policy_uri.value().as_ref().unwrap() || i.security_mode!=self.security_mode{
                        continue;
                    }
                    //we have the right security policy so we want to add the uri of credentials
                    for user_token_policy in &i.user_identity_tokens{
                        if user_token_policy.token_type==UserTokenType::USERNAME{
                            self.policy_id_user=user_token_policy.policy_id.clone();
                            let sec_uri:&str;
                            match user_token_policy.security_policy_uri.value().as_ref() {
                                Some(uri)=>sec_uri=uri,
                                None=>sec_uri=security_policy.policy_uri,
                            }
                            self.security_policy_uri_token_user=SecurityPolicyUri::get_security_policy_uri(sec_uri);
                        }
                        if user_token_policy.token_type==UserTokenType::ANONYMOUS{
                            self.policy_id_anon=user_token_policy.policy_id.clone();
                            let sec_uri:&str;
                            match user_token_policy.security_policy_uri.value().as_ref() {
                                Some(uri)=>sec_uri=uri,
                                None=>sec_uri=security_policy.policy_uri,
                            }
                            self.security_policy_uri_token_anon=SecurityPolicyUri::get_security_policy_uri(sec_uri);
                        }
                        if user_token_policy.token_type==UserTokenType::CERTIFICATE{
                            self.policy_id_cert=user_token_policy.policy_id.clone();
                            let sec_uri:&str;
                            match user_token_policy.security_policy_uri.value().as_ref() {
                                Some(uri)=>sec_uri=uri,
                                None=>sec_uri=security_policy.policy_uri,
                            }
                            self.security_policy_uri_token_cert=SecurityPolicyUri::get_security_policy_uri(sec_uri);
                        }
                    }
                    break;
                }

            },
            _ => {}
        }
    }

    pub (crate) fn update_response(msg: &Msg)->String {        
        match msg{
            Msg::ServiceFault(_m)=>{
                Handle::R_SERVICE_FAULT.to_string()
            },
            Msg::ErrorMessage(_m)=>{
                Handle::R_ERR.to_string() 
            },
            Msg::AckowledgeMessage(_)=>{
                Handle::R_ACK.to_string()
            },
            Msg::HelloMessage(_m)=>{
                Handle::R_HELLO.to_string()
            },
            Msg::OpenSecureChannelRequest(_m)=>{
                Handle::R_OPEN_REQ.to_string()
            },
            Msg::OpenSecureChannelResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_OPEN_REPNOK.to_string()
                }
                Handle::R_OPEN_REPOK.to_string()
            },
            Msg::GetEndPointsRequest(_m)=>{
                Handle::R_GEP_REQ.to_string()
            },
            Msg::GetEndPointsResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_GEP_REPNOK.to_string()
                }
                Handle::R_GEP_REPOK.to_string()
            },
            Msg::CloseSecureChannelRequest(_m)=>{
                Handle::R_CLOSE.to_string()
            },
            Msg::CreateSessionRequest(_m)=>{
                Handle::R_CREATE_SESS_REQ.to_string()
            },
            Msg::CreateSessionResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_CREATE_SESS_REPNOK.to_string()
                }
                Handle::R_CREATE_SESS_REPOK.to_string()
            },
            Msg::RevHelloMessage(_m)=>{
                Handle::R_REV_HELLO.to_string()
            },
            Msg::CloseSessionRequest(_m)=>{
                Handle::R_CLO_SESS_REQ.to_string()
            },
            Msg::CloseSessionResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_CLO_SESS_REPNOK.to_string()
                }
                Handle::R_CLO_SESS_REPOK.to_string()
            },
            Msg::ActiveSessionRequest(_m)=>{
                Handle::R_ACTIVE_SESS_REQ.to_string()
            },
            Msg::ActiveSessionResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_ACTIVE_SESS_REPNOK.to_string()
                }
                Handle::R_ACTIVE_SESS_REPOK.to_string()
            },
            Msg::ReadRequest(_m)=>{
                Handle::R_READ_REQ.to_string()
            },
            Msg::ReadResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_READ_REPNOK.to_string()
                }
                Handle::R_READ_REPOK.to_string()
            },
            Msg::NullSize(_m)=>{
                Handle::R_NULL_SIZE.to_string()
            },
            Msg::WriteRequest(_m)=>{
                Handle::R_WRITE_REQ.to_string()
            },
            Msg::WriteResponse(m)=>{
                if m.response_header.service_result.get_value()!=StatusCode::Good{
                    return Handle::R_WRITE_REPNOK.to_string()
                }
                Handle::R_WRITE_REPOK.to_string()
            },

        }
    }


    pub fn pre_send_update(&mut self, msg:&mut Vec<u8>){
        let msg_type=msg::get_type(msg);
        match msg_type{ 
            MessageType::ACK | MessageType::ERR | MessageType::RHE| MessageType::HEL =>{
                

            },
            //asymetric encryption
            MessageType::OPN=>{
                msg::set_sequence_number(msg, self.sequence_number);
                msg::set_request_id(msg, self.request_id);
                if self.c_chunk {
                    msg::set_intermediate_chunk(msg);
                    self.c_chunk=false; 
                }
                
            },
            //symmetric encryption
            _ =>{
                msg::set_secure_channel_id(msg, self.secure_channel_id);
                if self.shift_secure_token_id{
                    msg::set_secure_token_id(msg,self.token_id+60);
                    self.shift_secure_token_id=false;
                }else{
                    msg::set_secure_token_id(msg, self.token_id);
                }
                msg::set_sequence_number(msg, self.sequence_number);
                msg::set_request_id(msg, self.request_id);
                if self.c_chunk{
                    msg::set_intermediate_chunk(msg);
                    self.c_chunk=false; 
                }
            },
        }
        self.sequence_number += 1;
        self.request_id += 1;
    }
    pub(crate) fn update_internal(&mut self, msg:&Msg){
        match msg{ 
            //asymetric encryption
            Msg::OpenSecureChannelRequest(_)=>{
                self.client_nonce = ByteString::from(msg.get_nonce().clone());
            },
            //symmetric encryption
            _ =>{
            },
        }
    }

    pub (crate) fn translate_from_abstract_to_object(&mut self,endpoint_url: &UaString, message: &str, security_policy:&mut  SecurityPolicy,channel_timeout:&u32,session_timeout:&f64,receiver_certificate_thumbprint:&ByteString, server_public_key:Option<&PublicKey>,server_certificate:Option<&ByteString>) -> Option<Msg> {
        match message {
            Handle::HELLO => Some(Msg::HelloMessage(HelloMessage::build(endpoint_url))),
            Handle::OPN_REQ => Some(Msg::OpenSecureChannelRequest(OpenSecureChannelRequest::build(
                &self.sender_certificate,
                receiver_certificate_thumbprint,
                self.security_mode,
                &security_policy,
                *channel_timeout,
            ))),
            Handle::OPN_REQ_WRONG =>{
                self.security_false=true;
                Some(Msg::OpenSecureChannelRequest(OpenSecureChannelRequest::build(
                &self.sender_false_certificate,
                receiver_certificate_thumbprint,
                self.security_mode,
                &security_policy,
                *channel_timeout,
            )))},
            Handle::OPEN_REQ_C_CHUNK=>{
                self.c_chunk=true;
                Some(Msg::OpenSecureChannelRequest(OpenSecureChannelRequest::build(
                    &self.sender_certificate,
                    receiver_certificate_thumbprint,
                    self.security_mode,
                    &security_policy,
                    *channel_timeout,
                )))
            }
            Handle::CLO_REQ => Some(Msg::CloseSecureChannelRequest(CloseSecureChannelRequest::build())),
            Handle::GET_ENDPOINT_REQ => Some(Msg::GetEndPointsRequest(GetEndPointsRequest::build(
                self.secure_channel_id,
                self.token_id,
            ))),
            Handle::CREATE_SESS => {
                Some(Msg::CreateSessionRequest(CreateSessionRequest::build(endpoint_url,&self.sender_certificate,*session_timeout)))
            },
            Handle::CREATE_SESS_SEC_TOKEN_ID => {
                self.shift_secure_token_id=true;
                Some(Msg::CreateSessionRequest(CreateSessionRequest::build(endpoint_url,&self.sender_certificate,*session_timeout)))
            },
            Handle::CLOSE_SESS => {
                Some(Msg::CloseSessionRequest(CloseSessionRequest::build(&self.authentication_token)))
            }
            Handle::ACTIVE_SESS => {
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key,&self.server_nonce,server_certificate,&self.user_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_user,&self.policy_id_user,false,true,false)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }
                
            },
            Handle::ACTIVE_SESS_SEC_TOKEN_ID => {
                self.shift_secure_token_id=true;
                
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key,&self.server_nonce,server_certificate,&self.user_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_user,&self.policy_id_user,false,true,false)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }
            },
            Handle::SET_SEC_MODE_NONE =>{
                self.security_mode=MessageSecurityMode::NONE;
                None
            },
            Handle::ACTIVE_SESS_ANON =>{
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key,&self.server_nonce,server_certificate,&self.user_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_anon,&self.policy_id_anon,true,false,false)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }

            },
            Handle::ACTIVE_SESS_WRONG_USER =>{
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key,&self.server_nonce,server_certificate,&self.user_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_user,&self.policy_id_user,false,false,false)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }
                
            },
            Handle::ACTIVE_SESS_CERT =>{
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key,&self.server_nonce,server_certificate,&self.user_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_cert,&self.policy_id_cert,false,false,true)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }
                
            },
            Handle::ACTIVE_SESS_WRONG_CERT =>{
                if let (Some(server_public_key),Some(server_certificate)) = (server_public_key,server_certificate){
                    Some(Msg::ActiveSessionRequest(ActiveSessionRequest::build(server_public_key,&self.private_key_false,&self.server_nonce,server_certificate,&self.user_false_certificate,&self.authentication_token, security_policy,self.security_policy_uri_token_cert,&self.policy_id_cert,false,false,true)))
                }else{
                    panic!("you must obtain a certificate for you target before sending messages to her. Please use get certificate")
                }  
            },
            Handle::READ_REQ =>{
                Some(Msg::ReadRequest(ReadRequest::build(&self.authentication_token,&self.target_node)))
            },
            Handle::WRITE_REQ =>{
                Some(Msg::WriteRequest(WriteRequest::build(&self.authentication_token,&self.target_node,&self.target_node_value)))
            },
            Handle::NULL_SIZE=>{
                Some(Msg::NullSize(NullSize::build(
                    &self.sender_certificate,
                    receiver_certificate_thumbprint,
                    self.security_mode,
                    *channel_timeout,
                )))
            }
            _ => panic!("unreconized message"),
        }
    }

}

#[cfg(feature = "python")]
pub (crate) mod python {
    extern crate cpython;
    use std::cell::RefCell;
    use cpython::{py_class, FromPyObject, PyResult, Python};
    use super::Handle;

    pub struct RefCellHandle(pub(crate) RefCell<Handle<'static>>);
    impl<'a> FromPyObject<'a> for RefCellHandle {
        fn extract(py: Python, obj: &'a cpython::PyObject) -> PyResult<Self> {
            let arg = obj.extract::<cpython::PyTuple>(py)?;
            let key_path = arg.get_item(py, 0).extract::<String>(py)?;//"uaexpert_key.der";
            let false_key_path = arg.get_item(py, 1).extract::<String>(py)?;//"uaexpert_key.der";
            let own_cert_path = arg.get_item(py, 2).extract::<String>(py)?;//"cert_server_ctt.der";
            let own_false_cert_path = arg.get_item(py, 3).extract::<String>(py)?;//"cert_server_ctt.der";
            let usr_cert_path = arg.get_item(py, 4).extract::<String>(py)?;//"cert_server_ctt.der";
            let usr_false_cert_path = arg.get_item(py, 5).extract::<String>(py)?;//"cert_server_ctt.der";
            let security_mode = arg.get_item(py, 6).extract::<u32>(py)?;//"cert_server_ctt.der";
            Ok(RefCellHandle(RefCell::new(
                Handle::new_basic256_sha256(&key_path,&false_key_path,&own_cert_path,&own_false_cert_path,&usr_cert_path,&usr_false_cert_path,security_mode),
            )))
        }
    }

    py_class!(pub class Mapper |py| {
        data handle: RefCellHandle;

        def __new__(_cls, arg: RefCellHandle) -> PyResult<Mapper> {
            Mapper::create_instance(py, arg)
        }

        def submit_word(&self ,socket_addr:String,messages:Vec<String>,target_index:usize,timeout:u64,nb_messages:Vec<usize>,known_no_resp:Vec<usize>) -> PyResult<Vec<String>> {
            let messages:Vec<&str> = messages.iter().map(|s| &**s).collect();
            let res=self.handle(py).0.borrow_mut().submit_word(socket_addr,messages,target_index,timeout,nb_messages,known_no_resp);
            Ok(res)
        }

        def get_server_certificate(&self ,socket_addr:String,timeout:u64) ->PyResult<usize> {
            self.handle(py).0.borrow_mut().get_server_certificate(socket_addr,timeout);
            Ok(0)
        }

        def set_target_node(&self,type_:u8,namespace:u16,id:&str)->PyResult<usize>{
            self.handle(py).0.borrow_mut().set_target_node(type_,namespace,id);
            Ok(0)
        }
        def set_target_node_value(&self,type_:u8,value:&str)->PyResult<usize>{
            self.handle(py).0.borrow_mut().set_target_node_value(type_,value);
            Ok(0)
        }

    });
}

//Wrapping for learnlib
#[cfg(feature = "java")]
pub(crate) mod java{
    use jni::JNIEnv;
    use jni::objects::{JClass, JString, JObject, JValue};
    use jni::sys::{jlong,jint, jintArray,jobjectArray};
    use super::Handle;
    type JavaResult<T>=Result<T,jni::errors::Error>;
    pub fn get_raw<T>(v:T)->i64{
        //We need the data to be on the heap. So we first declare it with a Box pointer.
        let this: Box<T> = Box::new(v);
        //We got the raw pointer of our data
        let this: *mut T = Box::into_raw(this);
        //we convert it to i64. This is the address of the data v.
        this as i64
    }

    fn get_handler<'a>(env: &'a JNIEnv,obj: &'a JObject)-> *mut Handle<'a>{
        //get the long array that is the address of the mapper
        let mapper_raw:JValue= env.get_field(*obj,"mapper","J").expect("Can not access mapper when trying to get certificate");
        //get the raw value
        let mapper:i64=mapper_raw.j().expect("Can not access mapper when trying to get certificate");
        mapper as *mut Handle
    }
    fn get_int_array(env:&JNIEnv,array:jintArray)->Vec<usize>{
        let mut ret:Vec<jint>;
        if !array.is_null(){
            let size:usize=env.get_array_length(array).expect("huhu") as usize;
            ret=vec![0;size];
            env.get_int_array_region(array,0,&mut ret).unwrap();
        }
        else{
            ret=vec![];
        }
        ret.iter().map(|i:&jint|*i as usize).collect()
    }

    fn vec_string_to_java(env:&JNIEnv,array:Vec<String>,prefix_length:jint)->JavaResult<jobjectArray>{
        let ret=env.new_object_array(array.len() as i32-prefix_length,"java/lang/String",jni::objects::JObject::null())?;
        for (i,string) in array[prefix_length as usize..].iter().enumerate(){
            let jstr: JString = env.new_string(string).unwrap();
            env.set_object_array_element(ret,i as i32,jstr)?;
        }
        Ok(ret)
    }

    fn get_array_string(env:&JNIEnv,array:jobjectArray)->JavaResult<Vec<String>>{
        let size:usize=env.get_array_length(array)? as usize;
        let mut ret: Vec<String>=Vec::with_capacity(size);
        for i in 0..size{
            let temp= env.get_object_array_element(array,i as i32)?;
            let temp=temp.into_raw();
            unsafe{
                let temp=JString::from_raw(temp);
                ret.push(env.get_string(temp).expect("Couldn't get java string!").into());
            }
        }
        Ok(ret)
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_learner_OracleMapper_init(env: JNIEnv,_class: JClass, key_path:JString, wrong_key_path: JString, cert_path: JString, wrong_cert_path :JString, user_cert_true_path: JString, user_cert_wrong_path: JString,mode :jint)->jlong{
        let key_path: String =
            env.get_string(key_path).expect("Couldn't get java string!").into();
        let wrong_key_path: String =
            env.get_string(wrong_key_path).expect("Couldn't get java string!").into();
        let cert_path: String =
            env.get_string(cert_path).expect("Couldn't get java string!").into();
        let wrong_cert_path: String =
            env.get_string(wrong_cert_path).expect("Couldn't get java string!").into();
        let user_cert_true_path: String =
            env.get_string(user_cert_true_path).expect("Couldn't get java string!").into();
        let user_cert_wrong_path: String =
            env.get_string(user_cert_wrong_path).expect("Couldn't get java string!").into();
        let mapper = Handle::new_basic256_sha256(&key_path,&wrong_key_path,&cert_path,&wrong_cert_path,&user_cert_true_path,&user_cert_wrong_path,mode as u32);
        get_raw(mapper)

    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_learner_OracleMapper_get_1server_1certificate(env: JNIEnv,obj: JObject, dest:JString, timeout:jint){
        let dest: String =env.get_string(dest).expect("Couldn't get java string!").into();
        // let mapper_raw:JValue= env.get_field(obj,"mapper","J").expect("Can not access mapper when trying to get certificate");
        // let mapper=mapper_raw.j().expect("Can not access mapper when trying to get certificate");
        let mapper = get_handler(&env,&obj);
        unsafe{
            (*mapper).get_server_certificate(dest,timeout as u64)
        }
    }
    

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_learner_OracleMapper_submit_1word(env: JNIEnv,obj: JObject, dest:JString,letters: jobjectArray,target_index:jint, timeout:jint,prefix_length:jint,nb_messages:jintArray,known_no_resp:jintArray)->jobjectArray{
        
        let letters:Vec<String>=get_array_string(&env, letters).unwrap();
        let known_no_resp:Vec<usize>=get_int_array(&env,known_no_resp);
        let nb_messages:Vec<usize>=get_int_array(&env,nb_messages);
        let dest:String=env.get_string(dest).expect("Couldn't get java string!").into();
        let target_index:usize=target_index as usize;
        let letters = letters.iter().map(|s| &**s).collect();
        let handle:*mut Handle=get_handler(&env, &obj);
        unsafe{
            if handle.is_null(){
                panic!("mapper handler is empty in submit word");
            }else{
                let ret=(*handle).submit_word(dest,letters,target_index,timeout as u64,nb_messages,known_no_resp);
                vec_string_to_java(&env,ret,prefix_length).unwrap()
            }
        }
        
    }

}
