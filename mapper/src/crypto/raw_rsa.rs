use std::ops::DerefMut;

use openssl::{bn::{BigNumRef, BigNumContext}, rsa::Rsa, pkey::Public};
use crate::{result_prelude::*, crypto::random::bytes};
use super::pkey::PublicKey;
use openssl::bn::BigNum;

pub(crate) fn raw_rsa(key:&PublicKey,data:&[u8])->MapperResult<Vec<u8>>{
    let rsa_key: Rsa<Public>;
    let to_encrypt:BigNum;
    let mut result:BigNum;
    let mut ctx:BigNumContext;
    match key.rsa(){
        Ok(k)=>{
            rsa_key=k;
        }
        Err(_)=>{
            return Err(MapperError::new(MapperErrorKind::RawRsaError, "failed to perform raw rsa"));
        }
    }
    match BigNum::from_slice(data){
        Ok(bigint_data)=>{
            to_encrypt=bigint_data;
        }
        Err(_)=>{
            return Err(MapperError::new(MapperErrorKind::RawRsaError, "error converting to data to number for raw_rsa"));
        }
    }
    let e: &BigNumRef = rsa_key.e();
    let n : &BigNumRef = rsa_key.n();
    match BigNum::new(){
        Ok(bigint)=>{
            result=bigint;
        }
        Err(_)=>{
            return Err(MapperError::new(MapperErrorKind::RawRsaError, "error converting to data to number for raw_rsa"));
        }
    }
    match BigNumContext::new(){
        Ok(bigint)=>{
            ctx=bigint;
        }
        Err(_)=>{
            return Err(MapperError::new(MapperErrorKind::RawRsaError, "error converting to data to number for raw_rsa"));
        }
    }
    let ret=result.mod_exp(&to_encrypt,e , n ,ctx.deref_mut());
    if let Err(_)=ret{
        return Err(MapperError::new(MapperErrorKind::RawRsaError, "failed to perform raw rsa"));
    }
    
    Ok(result.to_vec())
}


pub(crate) fn raw_rsa_pkcs15(key:&PublicKey,data:&[u8])->MapperResult<Vec<u8>>{
    let key_size=key.size();
    let mut to_encrypt: Vec<u8>=vec![0x00,0x02];
    let padding_size:usize=key_size-(data.len()+3) ;
    let mut padding:Vec<u8>=vec![0;padding_size-1];
    bytes(&mut padding);
    // to_encrypt.extend_from_slice(&vec![0x97;key_size-(data.len()+3)]);
    for i in &mut padding{
        if *i==0{
            *i=1
        }
    }
    to_encrypt.extend_from_slice(&padding.to_vec());
    to_encrypt.push(0x01);
    to_encrypt.push(0x00);
    to_encrypt.extend_from_slice(&data);
    raw_rsa(key, &to_encrypt)
}

pub(crate) fn raw_rsa_per_block(key:&PublicKey,data:&[u8],custom_padding:&[u8])->MapperResult<Vec<u8>>{
    let nb_block;
    let block_size=key.size();
    let custom_padding_len=custom_padding.len();
    if data.len()%(key.size()-custom_padding.len())==0{
        nb_block=data.len()/(key.size()-custom_padding.len());
    }else{
        nb_block=data.len()/(key.size()-custom_padding.len())+1;
    }
    let mut result:Vec<u8>= Vec::new();
    let mut start:usize=0;
    let mut end:usize;
    let size=block_size-custom_padding_len;
    for _i in 0..nb_block{
        let mut block_data:Vec<u8>=Vec::new();
        block_data.extend_from_slice(custom_padding);
        end =start+size;
        if end>=data.len(){
            block_data.extend_from_slice(&data[start..]);
        }else{
            block_data.extend_from_slice(&data[start..end]);
        }
        let block=raw_rsa(key, &block_data)?;
        result.extend_from_slice(&block);
        start=end;
    }
    Ok(result)
    
}


/****************************************************************************************************/
/****************************************************************************************************/
/****************************************************************************************************/
/****************************Bleichen baucher oracle detection***************************************/
/****************************************************************************************************/
/****************************************************************************************************/
pub(crate) struct BleichenbaucherOracleDetector{
    pub(crate) valid:Vec<u8>,
    pub(crate) missing02:Vec<u8>,
    pub(crate) no_zero:Vec<u8>,
    pub(crate) short_padding: Vec<u8>,
    pub(crate) wrong_plaintext:Vec<u8>,
    pub(crate) sig_short:Vec<u8>,
    pub(crate) sig_long:Vec<u8>,
    pub(crate) sig_rand:Vec<u8>,
}
impl BleichenbaucherOracleDetector{
    pub(crate) const NAME:[&'static str; 8]=["valid","missing02","no_zero","short_padding","wrong_plaintext","sig short","sig long","sig rand"];
    fn new()->Self{
        BleichenbaucherOracleDetector{
            valid:Vec::new(),
            missing02:Vec::new(),
            no_zero:Vec::new(),
            short_padding:Vec::new(),
            wrong_plaintext:Vec::new(),
            sig_short:Vec::new(),
            sig_long:Vec::new(),
            sig_rand:Vec::new(),
        }
    }

    pub(crate) fn bleichen_baucher_test(key:&PublicKey,msg:&[u8],security_policy: &mut SecurityPolicy)->MapperResult<Self>{
        //need to do per block 
        let mut result=BleichenbaucherOracleDetector::new();
        let mut padding:Vec<u8>=vec![0;8];
        bytes(&mut padding);
        for i in &mut padding{
            if *i==0{
                *i=1
            }
        }
        let mut offset;
        //correct encryption
        let mut valid_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&valid_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut valid_msg,11,offset,false,false,false)?;
        let mut valid_data_padding:Vec<u8>=vec![0x00,0x02];
        valid_data_padding.extend_from_slice(&padding);
        valid_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&valid_msg[offset..],&valid_data_padding)?;
        valid_msg.truncate(offset);
        valid_msg.extend_from_slice(&encrypted);
        result.valid=valid_msg;

        //missing02
        let mut missing02_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&missing02_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut missing02_msg,11,offset,false,false,false)?;
        let mut missing02_data_padding:Vec<u8>=vec![0x00,0x03];
        missing02_data_padding.extend_from_slice(&padding);
        missing02_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&missing02_msg[offset..],&missing02_data_padding)?;
        missing02_msg.truncate(offset);
        missing02_msg.extend_from_slice(&encrypted);
        result.missing02=missing02_msg;

        // //no zero to end padding
        let mut zero_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&zero_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut zero_msg,11,offset,false,false,false)?;
        let mut zero_data_padding:Vec<u8>=vec![0x00,0x03];
        zero_data_padding.extend_from_slice(&padding);
        let encrypted=raw_rsa_per_block(key,&zero_msg[offset..],&zero_data_padding)?;
        zero_msg.truncate(offset);
        zero_msg.extend_from_slice(&encrypted);
        result.no_zero=zero_msg;

        // short padding
        let mut short_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&short_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut short_msg,6,offset,false,false,false)?;
        let mut short_data_padding:Vec<u8>=vec![0x00,0x02];
        short_data_padding.extend_from_slice(&padding[..3]);
        short_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&short_msg[offset..],&short_data_padding)?;
        short_msg.truncate(offset);
        short_msg.extend_from_slice(&encrypted);
        result.short_padding=short_msg;


        // //wrong_plaintext
        let mut wrong_plaintext_msg=vec![0;msg.len()];
        bytes(&mut wrong_plaintext_msg);
        set_padding_signature_bleichenbaucher(security_policy,&mut wrong_plaintext_msg,11,offset,false,false,false)?;
        let mut wrong_plaintext_data_padding:Vec<u8>=vec![0x00,0x02];
        wrong_plaintext_data_padding.extend_from_slice(&padding);
        wrong_plaintext_data_padding.push(0x00);
        // offset=crate::msg::get_offset_to_encrypt(&msg.to_vec())?;
        let encrypted=raw_rsa_per_block(key,&wrong_plaintext_msg[offset..],&wrong_plaintext_data_padding)?;
        wrong_plaintext_msg.truncate(offset);
        wrong_plaintext_msg.extend_from_slice(&encrypted);
        result.wrong_plaintext=wrong_plaintext_msg;

        //sig short
        let mut sig_short_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&sig_short_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut sig_short_msg,11,offset,true,false,false)?;
        let mut sig_short_data_padding:Vec<u8>=vec![0x00,0x02];
        sig_short_data_padding.extend_from_slice(&padding);
        sig_short_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&sig_short_msg[offset..],&sig_short_data_padding)?;
        sig_short_msg.truncate(offset);
        sig_short_msg.extend_from_slice(&encrypted);
        result.sig_short=sig_short_msg;

        //sig long
        let mut sig_long_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&sig_long_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut sig_long_msg,11,offset,false,true,false)?;
        let mut sig_long_data_padding:Vec<u8>=vec![0x00,0x02];
        sig_long_data_padding.extend_from_slice(&padding);
        sig_long_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&sig_long_msg[offset..],&sig_long_data_padding)?;
        sig_long_msg.truncate(offset);
        sig_long_msg.extend_from_slice(&encrypted);
        result.sig_long=sig_long_msg;

        //sig rand
        let mut sig_rand_msg=msg.to_vec();
        offset=crate::msg::get_offset_to_encrypt(&sig_rand_msg)?;
        set_padding_signature_bleichenbaucher(security_policy,&mut sig_rand_msg,11,offset,false,false,true)?;
        let mut sig_rand_data_padding:Vec<u8>=vec![0x00,0x02];
        sig_rand_data_padding.extend_from_slice(&padding);
        sig_rand_data_padding.push(0x00);
        let encrypted=raw_rsa_per_block(key,&sig_rand_msg[offset..],&sig_rand_data_padding)?;
        sig_rand_msg.truncate(offset);
        sig_rand_msg.extend_from_slice(&encrypted);
        result.sig_rand=sig_rand_msg;
        Ok(result)

    }

    pub(crate) fn to_vec(&self)->Vec<&Vec<u8>>{
        vec![&self.valid,&self.missing02,&self.no_zero,&self.short_padding,&self.wrong_plaintext,&self.sig_short,&self.sig_long,&self.sig_rand]
    }
}

use crate::msg::{write_bytes,set_size_custom,minimum_padding};
use super::security_policy::*;
//for bleichen  baucher we need to modify the padding in order to have a complete number of block (opc ua spec).
//this can only be used with rsapkcs15sha1
fn set_padding_signature_bleichenbaucher(
    security_policy: &mut SecurityPolicy,
    body: &mut Vec<u8>,
    custom_padding_len: usize,
    offset:usize,
    signature_short:bool,
    signature_long:bool,
    random_signature:bool,
) ->MapperResult<()>{
    if signature_short && signature_long || signature_short && random_signature || signature_long && random_signature{
        panic!("you can apply signature modification only one time for padding signature bleichencaucher");
    }
    let cipher_text_block_size: usize;
    let plain_text_block_size: usize;
    let signature_size: usize;

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
    let size = encrypter_decrypter.encrypter.encrypt_len(body).unwrap();
    (plain_text_block_size, cipher_text_block_size) = (
        size-custom_padding_len,
        size,
    );
    if signature_long{
        signature_size = signer.signature_size()+10;
    }else if signature_short{
        signature_size = signer.signature_size()-10;
    }else{
        signature_size = signer.signature_size();
    }
    let (padding_size, minimum_padding) = padding_size(
        cipher_text_block_size,
        plain_text_block_size,
        signature_size,
        body[offset..].len(),
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
        size = offset
                + ((body[offset..].len() + signature_size) / plain_text_block_size)
                    * cipher_text_block_size;
        
        set_size_custom(body, size);
        
        
        if signature_long ||signature_short || random_signature{
            let mut signature=vec![0;signature_size];
            bytes(&mut signature);
            body.extend_from_slice(&signature);
        }else{
            let signature:Vec<u8>;
            signature=security_policy.asymmetric_signature_algorithm.as_mut().unwrap().sign(body);
            body.extend_from_slice(&signature); 
        }
        
    }
    Ok(())
}


fn padding_size(
    key_length: usize,
    plain_text_block_size: usize,
    signature_size: usize,
    bytes_to_writes_encrypted: usize,
) -> (usize, usize) {
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
}
