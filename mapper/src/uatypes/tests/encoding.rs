#[cfg(test)]
mod tests {
    use derive_macro::Serialize;

    use crate::uatypes::guid::Guid;
    use crate::*;
    #[test]
    fn guid_serialize() {
        let guid: Vec<u8> = Guid::from("72962B91-FA75-4AE6-8D28-B404DC7DAF63").serialize();
        let result: Vec<u8> = vec![
            0x91, 0x2B, 0x96, 0x72, 0x75, 0xFA, 0xE6, 0x4A, 0x8D, 0x28, 0xB4, 0x04, 0xDC, 0x7D,
            0xAF, 0x63,
        ];
        assert_eq!(result, guid);
    }

    use crate::uatypes::string::UaString;
    #[test]
    fn string_serialize() {
        let hello: Vec<u8> = UaString::from("opc.tcp://localhost:4840").serialize();
        let result: Vec<u8> = vec![
            0x18, 0x00, 0x00, 0x00, 0x6f, 0x70, 0x63, 0x2e, 0x74, 0x63, 0x70, 0x3a, 0x2f, 0x2f,
            0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x68, 0x6f, 0x73, 0x74, 0x3a, 0x34, 0x38, 0x34, 0x30,
        ];
        assert_eq!(result, hello);
        let hello: Vec<u8> = UaString::from("Hot水").serialize();
        let result: Vec<u8> = vec![0x06, 0x00, 0x00, 0x00, 0x48, 0x6F, 0x74, 0xe6, 0xb0, 0xb4];
        assert_eq!(result, hello);
    }

    #[test]
    fn boolean_serialize() {
        let bool: Vec<u8> = true.serialize();
        let result: Vec<u8> = vec![0x01];
        assert_eq!(result, bool);
        let bool: Vec<u8> = false.serialize();
        let result: Vec<u8> = vec![0x00];
        assert_eq!(result, bool);
    }

    //Test for int. All integer implementation are the same
    #[test]
    fn i32_serialize() {
        let int: Vec<u8> = 1_000_000_000.serialize();
        let result: Vec<u8> = vec![0x00, 0xca, 0x9a, 0x3b];
        assert_eq!(result, int);
    }

    //Test for float. All float implementation are the same (even DateTime)
    #[test]
    fn float_serialize() {
        let float: f32 = -6.5;
        let float = float.serialize();
        let result: Vec<u8> = vec![0x00, 0x00, 0xd0, 0xc0];
        assert_eq!(result, float);
    }

    use crate::uatypes::byte_string::ByteString;
    #[test]
    fn byte_string_serialize() {
        let bstring: Vec<u8> = ByteString::from(vec![0x04, 0x03]).serialize();
        let result: Vec<u8> = vec![0x02, 0x00, 0x00, 0x00, 0x04, 0x03];
        assert_eq!(result, bstring);
        let bstring = ByteString::new().serialize();
        let result: Vec<u8> = vec![0xff, 0xff, 0xff, 0xff];
        assert_eq!(result, bstring);
    }

    //the example from OPC-UA is wrong about the encoding of Hot水 (72 is in decimal not hex).
    use crate::uatypes::node_id::{Identifier, NodeId};
    #[test]
    fn node_id_serialize() {
        let node: Vec<u8> = NodeId {
            namespace: 0,
            identifier: Identifier::Numeric(631),
        }
        .serialize();
        let result: Vec<u8> = vec![0x01, 0x00, 0x77, 0x02];
        assert_eq!(result, node);
        let node: Vec<u8> = NodeId {
            namespace: 0,
            identifier: Identifier::Numeric(72),
        }
        .serialize();
        let result: Vec<u8> = vec![0x00, 0x48];
        assert_eq!(result, node);
        let node: Vec<u8> = NodeId {
            namespace: 1,
            identifier: Identifier::String(UaString::from("Hot水")),
        }
        .serialize();
        let result: Vec<u8> = vec![
            0x03, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x48, 0x6F, 0x74, 0xe6, 0xb0, 0xb4,
        ];
        assert_eq!(result, node);
    }
    use crate::uatypes::diagnostic_info::*;
    use crate::uatypes::localized_text::LocalizedText;

    #[test]
    fn diagnostic_info_serialize() {
        let diag1 = DiagnosticInfo {
            encoding_mask: 0,
            symbolic_id: 01,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xffff,
            inner_diagnostic_info: None,
        };
        let result: Vec<u8> = vec![0x01, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(result, diag1.serialize());
        let diag2 = DiagnosticInfo {
            encoding_mask: 0,
            symbolic_id: 1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xffff,
            inner_diagnostic_info: Some(Box::new(diag1)),
        };
        let result: Vec<u8> = vec![
            0x01 | 0x40,
            0x01,
            0x00,
            0x00,
            0x00,
            0x01,
            0x01,
            0x00,
            0x00,
            0x00,
        ];
        assert_eq!(result, diag2.serialize());
        let diag3 = DiagnosticInfo {
            encoding_mask: 0,
            symbolic_id: 1,
            namespace_uri: -1,
            locale: -1,
            localized_text: -1,
            additional_info: UaString::new(),
            inner_status_code: 0xffff,
            inner_diagnostic_info: Some(Box::new(diag2)),
        };
        let result: Vec<u8> = vec![
            0x01 | 0x40,
            0x01,
            0x00,
            0x00,
            0x00,
            0x01 | 0x40,
            0x01,
            0x00,
            0x00,
            0x00,
            0x01,
            0x01,
            0x00,
            0x00,
            0x00,
        ];
        assert_eq!(result, diag3.serialize());
    }
    use crate::uatypes::qualified_name::QualifiedName;
    #[test]
    fn qualified_name_serialize() {
        let qname = QualifiedName::new(72, UaString::from("Hot水"));
        let result: Vec<u8> = vec![
            0x48, 0x00, 0x06, 0x00, 0x00, 0x00, 0x48, 0x6F, 0x74, 0xe6, 0xb0, 0xb4,
        ];
        assert_eq!(result, qname.serialize());
    }

    #[test]
    fn localized_text_serialize() {
        let ltext = LocalizedText::new(UaString::from("CN"), UaString::from("Hot水"));
        let result: Vec<u8> = vec![
            0x03, 0x02, 0x00, 0x00, 0x00, 0x43, 0x4e, 0x06, 0x00, 0x00, 0x00, 0x48, 0x6F, 0x74,
            0xe6, 0xb0, 0xb4,
        ];
        assert_eq!(result, ltext.serialize());
    }
    use crate::uatypes::extension_object;
    use crate::uatypes::extension_object::ExtensionObject;
    #[test]
    fn extension_object_serialize() {
        let var = ExtensionObject {
            type_id: NodeId {
                namespace: 0,
                identifier: Identifier::Numeric(631),
            },
            encoding: extension_object::EncodingValue::NO_BODY,
            body: extension_object::ExtensionObjectBody::None,
        };
        let result: Vec<u8> = vec![0x01, 0x00, 0x77, 0x02, 0x00];
        assert_eq!(result, var.serialize());
    }

    //derive macro test
    #[derive(Serialize)]
    struct DeriveTest {
        a: i32,
        b: i32,
    }

    #[test]
    fn derive_serialize_test() {
        let derive_test = DeriveTest {
            a: 1_000_000_000,
            b: 1_000_000_000,
        };
        let serialized_struct = derive_test.serialize();
        let result: Vec<u8> = vec![0x00, 0xca, 0x9a, 0x3b, 0x00, 0xca, 0x9a, 0x3b];
        assert_eq!(result, serialized_struct);
    }

    //derive macro test with lifetime
    #[derive(Serialize)]

    pub struct DeriveTestLT<'a> {
        a: i32,
        b: Stime<'a>,
    }

    #[derive(Debug)]

    struct Stime<'a> {
        c: &'a i32,
    }

    impl<'a> Serialize for Stime<'a> {
        fn serialize(&self) -> Vec<u8> {
            self.c.serialize()
        }
    }

    #[test]
    fn derive_serialize_lifetime_test() {
        let derive_test = DeriveTestLT {
            a: 1_000_000_000,
            b: Stime { c: &1_000_000_000 },
        };
        let serialized_struct = derive_test.serialize();
        let result: Vec<u8> = vec![0x00, 0xca, 0x9a, 0x3b, 0x00, 0xca, 0x9a, 0x3b];
        assert_eq!(result, serialized_struct);
    }
}
