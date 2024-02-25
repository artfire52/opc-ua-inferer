#[cfg(test)]
mod tests {

    use derive_macro::{Deserialize, Serialize};

    use crate::{
        uatypes::{
            application_description::ApplicationDescription, byte_string::ByteString,
            diagnostic_info::DiagnosticInfo, localized_text::LocalizedText, string::UaString,
            user_identity_token::UserTokenPolicy,
        },
        *,
    };
    #[test]
    fn bool_decoding() {
        let bool = true.serialize();
        let (_, result) = bool::deserialize(&bool).unwrap();
        assert_eq!(true, result);
        let bool = false.serialize();
        let (_, result) = bool::deserialize(&bool).unwrap();
        assert_eq!(false, result)
    }

    #[test]
    fn i32_decoding() {
        let int32 = 32.serialize();
        let (_, result) = i32::deserialize(&int32).unwrap();
        assert_eq!(32, result);
    }

    #[test]
    fn i64_decoding() {
        let number = (32 as i64).serialize();
        let (_, result) = i64::deserialize(&number).unwrap();
        assert_eq!(32, result);
        let number = vec![0x00, 0x00, 0x00, 0x00, 0x80, 0x4f, 0x32, 0x41];
        let (_, result) = f64::deserialize(&number).unwrap();
        assert_eq!(1200000.0, result);
    }

    #[test]
    fn u32_decoding() {
        let uint32 = vec![0x70, 0x07, 0x00, 0x00];
        let (_, result) = u32::deserialize(&uint32).unwrap();
        assert_eq!(1904, result);
    }

    #[test]
    fn u16_decoding() {
        let int32 = (32 as u16).serialize();
        let (_, result) = u16::deserialize(&int32).unwrap();
        assert_eq!(32, result);
    }

    #[test]
    fn u8_decoding() {
        let int32 = (32 as u8).serialize();
        let (_, result) = u8::deserialize(&int32).unwrap();
        assert_eq!(32, result);
    }

    #[test]
    fn float_decoding() {
        let float = (32.5 as f32).serialize();
        let (_, result) = f32::deserialize(&float).unwrap();
        assert_eq!(32.5 as f32, result);
    }

    #[test]
    fn double_decoding() {
        let double = (32.5 as f64).serialize();
        let (_, result) = f64::deserialize(&double).unwrap();
        assert_eq!(32.5 as f64, result);
    }

    #[test]
    fn string_decoding() {
        let s = String::from("decode");
        let (_, result) = String::deserialize(&s.serialize()).unwrap();
        assert_eq!(s, result);
    }

    #[test]
    fn byte_decoding() {
        let bstring = ByteString::from(vec![0x04, 0x03]);
        let (_, result) = ByteString::deserialize(&bstring.serialize()).unwrap();
        assert_eq!(bstring, result);
        let bstring = ByteString::new();
        let (_, result) = ByteString::deserialize(&bstring.serialize()).unwrap();
        assert_eq!(bstring, result);
    }
    #[test]
    fn uastring_decoding() {
        let bstring = UaString::from("decode");
        let (_, result) = UaString::deserialize(&bstring.serialize()).unwrap();
        assert_eq!(bstring, result);
        let bstring = UaString::new();
        let (_, result) = UaString::deserialize(&bstring.serialize()).unwrap();
        assert_eq!(bstring, result);
    }

    use crate::uatypes::node_id::{Identifier, NodeId};
    #[test]
    fn node_id_serialize() {
        let node = NodeId {
            namespace: 0,
            identifier: Identifier::Numeric(631),
        };
        let (_, result) = NodeId::deserialize(&node.serialize()).unwrap();
        assert_eq!(result, node);
        let node = NodeId {
            namespace: 0,
            identifier: Identifier::Numeric(72),
        };
        let (_, result) = NodeId::deserialize(&node.serialize()).unwrap();
        assert_eq!(result, node);
        let node = NodeId {
            namespace: 1,
            identifier: Identifier::String(UaString::from("Hot水")),
        };
        let (_, result) = NodeId::deserialize(&node.serialize()).unwrap();
        assert_eq!(result, node);
    }

    #[test]
    fn diagnostic_info_deserialize() {
        let diag1 = DiagnosticInfo {
            encoding_mask: crate::uatypes::diagnostic_info::EncodingValue::SYMBOLIC_ID,
            symbolic_id: 1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xFFFFFFFF,
            inner_diagnostic_info: None,
        };
        let (_, result) = DiagnosticInfo::deserialize(&diag1.serialize()).unwrap();
        assert_eq!(result, diag1);
        let diag2 = DiagnosticInfo {
            encoding_mask: crate::uatypes::diagnostic_info::EncodingValue::SYMBOLIC_ID
                | crate::uatypes::diagnostic_info::EncodingValue::INNER_DIAGNOSTIC_INFO,
            symbolic_id: 1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xFFFFFFFF,
            inner_diagnostic_info: Some(Box::new(diag1)),
        };
        let (_, result) = DiagnosticInfo::deserialize(&diag2.serialize()).unwrap();
        assert_eq!(result, diag2);
        let diag3 = DiagnosticInfo {
            encoding_mask: crate::uatypes::diagnostic_info::EncodingValue::SYMBOLIC_ID
                | crate::uatypes::diagnostic_info::EncodingValue::INNER_DIAGNOSTIC_INFO,
            symbolic_id: 1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xFFFFFFFF,
            inner_diagnostic_info: Some(Box::new(diag2)),
        };
        let (_, result) = DiagnosticInfo::deserialize(&diag3.serialize()).unwrap();
        assert_eq!(result, diag3);
    }

    #[test]
    fn localized_text_deserialize() {
        let ltext = LocalizedText {
            encoding_mask: 03 as u8,
            locale: UaString::from("CN"),
            text: UaString::from("Hot水"),
        };
        let result: Vec<u8> = vec![
            0x03, 0x02, 0x00, 0x00, 0x00, 0x43, 0x4e, 0x06, 0x00, 0x00, 0x00, 0x48, 0x6F, 0x74,
            0xe6, 0xb0, 0xb4,
        ];
        let (_,result) = LocalizedText::deserialize(&result).unwrap();
        assert_eq!(result, ltext);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct DeriveTest {
        a: i32,
        b: i32,
    }
    #[test]
    fn derive_deserialize_test() {
        let derive_test = DeriveTest {
            a: 1_000_000_000,
            b: 1_000_000_000,
        };
        let serialized_struct = derive_test.serialize();
        let result: Vec<u8> = vec![0x00, 0xca, 0x9a, 0x3b, 0x00, 0xca, 0x9a, 0x3b];
        assert_eq!(result, serialized_struct);
        let (_, deserialized) = DeriveTest::deserialize(&serialized_struct).unwrap();
        assert_eq!(derive_test, deserialized);
    }


    #[test]
    fn derive_deserialize_vec() {
        let derive_test: Vec<i32> = vec![1_000_000_000, 1_000_000_000];
        let serialized_struct = derive_test.serialize();
        let result: Vec<u8> = hex::decode("0200000000ca9a3b00ca9a3b").unwrap(); 
        assert_eq!(result, serialized_struct);
        let (_, deserialized) = Vec::<i32>::deserialize(&serialized_struct).unwrap();
        assert_eq!(derive_test, deserialized);
        let derive_test: Vec<i32> = vec![];
        let serialized_struct = derive_test.serialize();
        let result: Vec<u8> = vec![0x00, 0x00, 0x00, 0x00];
        assert_eq!(result, serialized_struct);
        let (_, deserialized) = Vec::<i32>::deserialize(&serialized_struct).unwrap();
        assert_eq!(derive_test, deserialized);
    }

    #[test]
    fn deserialize_application_description() {
        let encoded_data=hex::decode("2000000075726e3a6f70656e36323534312e7365727665722e6170706c69636174696f6e14000000687474703a2f2f6f70656e36323534312e6f72670302000000656e220000006f70656e36323534312d6261736564204f5043205541204170706c69636174696f6e00000000ffffffffffffffff01000000180000006f70632e7463703a2f2f6c6f63616c686f73743a34383430").unwrap();
        let (_,application_description) = ApplicationDescription::deserialize(&encoded_data).unwrap();
        let aim_application_description = ApplicationDescription {
            application_uri: UaString::from("urn:open62541.server.application"),
            product_uri: UaString::from("http://open62541.org"),
            application_name: LocalizedText {
                encoding_mask: 03 as u8,
                locale: UaString::from("en"),
                text: UaString::from("open62541-based OPC UA Application"),
            },
            application_type: 0,
            gateway_server_uri: UaString::new(),
            discovery_policy_uri: UaString::new(),
            discovery_urls: vec![UaString::from("opc.tcp://localhost:4840")],
        };
        assert_eq!(aim_application_description, application_description);
    }

    #[test]
    fn deserialize_user_identity_token() {
        let encoded_data=hex::decode("1a0000006f70656e36323534312d616e6f6e796d6f75732d706f6c69637900000000ffffffffffffffffffffffff").unwrap();
        let user_identity_token = UserTokenPolicy {
            policy_id: UaString::from("open62541-anonymous-policy"),
            token_type: 0x00,
            issued_token_type: UaString::new(),
            issuer_endpoint_url: UaString::new(),
            security_policy_uri: UaString::new(),
        };
        let (_,result) = UserTokenPolicy::deserialize(&encoded_data).unwrap();
        assert_eq!(user_identity_token, result);
    }
}
