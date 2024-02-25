use derive_macro::Deserialize;

use crate::{
    uatypes::{
        byte_string::ByteString, date_time::DateTime, diagnostic_info::DiagnosticInfo,
        expanded_node_id::ExpandedNodeId, guid::Guid, localized_text::LocalizedText,
        node_id::NodeId, qualified_name::QualifiedName, status_code::StatusCode, string::UaString,
        xml_element::XmlElement,
    },
    Deserialize, Serialize,MapperResult
};
//This file is random do not even try to read it before this commment disappear.
//it would be a good to waste time
#[derive(Debug)]
pub (crate)  struct AdditionalHeaders {
    pub(crate) content: Vec<KeyValuePair>,
}

#[derive(Debug,Deserialize)]
pub (crate)  struct KeyValuePair {
    pub(crate) key: QualifiedName,
    pub(crate) value: BaseDataType,
}

impl AdditionalHeaders {
    pub fn deug() -> AdditionalHeaders {
        AdditionalHeaders {
            content: vec![KeyValuePair {
                key: QualifiedName {
                    namespace_index: 0,
                    name: UaString::from("oui"),
                },
                value: BaseDataType::String(UaString::from("non")),
            }],
        }
    }
}

#[derive(Debug)]
///<https://reference.opcfoundation.org/v104/Core/DataTypes/BaseDataType/>
pub (crate)  enum BaseDataType {
    Boolean(bool),
    String(UaString),
    DateTime(DateTime),
    Guid(Guid),
    ByteString(ByteString),
    XmlElement(XmlElement),
    NodeId(NodeId),
    ExpandedNodeId(ExpandedNodeId),
    StatusCode(StatusCode),
    QualifiedName(QualifiedName),
    LocalizedText(LocalizedText),
    // DataValue(DataValue<T>),
    DiagnosticInfo(DiagnosticInfo),
    // RsaEncryptedSecret(RsaEncryptedSecret),
    // EccEncryptedSecret(EccEncryptedSecret),
}

impl Serialize for BaseDataType {
    fn serialize(&self) -> Vec<u8> {
        match self {
            BaseDataType::Boolean(bool) => bool.serialize(),
            BaseDataType::String(ua_string) => ua_string.serialize(),
            BaseDataType::DateTime(date_time) => date_time.serialize(),
            BaseDataType::Guid(guid) => guid.serialize(),
            BaseDataType::ByteString(byte_string) => byte_string.serialize(),
            BaseDataType::XmlElement(xml_element) => xml_element.serialize(),
            BaseDataType::NodeId(node_id) => node_id.serialize(),
            BaseDataType::ExpandedNodeId(expanded_node_id) => expanded_node_id.serialize(),
            BaseDataType::StatusCode(code) => code.serialize(),
            BaseDataType::QualifiedName(qualified_name) => qualified_name.serialize(),
            BaseDataType::LocalizedText(localized_text) => localized_text.serialize(),
            // DataValue(DataValue<T>),
            BaseDataType::DiagnosticInfo(diagnostif_info) => diagnostif_info.serialize(),
            // RsaEncryptedSecret(RsaEncryptedSecret),
            // EccEncryptedSecret(EccEncryptedSecret),
        }
    }
}

impl Deserialize for BaseDataType {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        Ok((data, BaseDataType::Boolean(true)))
    }
}

impl Serialize for KeyValuePair {
    fn serialize(&self) -> Vec<u8> {
        let mut result = self.key.serialize();
        result.extend_from_slice(&self.value.serialize());
        result
    }
}


impl Serialize for AdditionalHeaders {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for el in self.content.iter() {
            result.extend_from_slice(&el.serialize());
        }
        result
    }
}

impl Deserialize for AdditionalHeaders {
    fn deserialize(data: &[u8]) -> MapperResult<(&[u8], Self)> {
        // let (data,key) = QualifiedName::deserialize(data);
        // let (data,value) = BaseDataType::deserialize(data);
        Ok((
            data,
            AdditionalHeaders {
                content: Vec::new(),
            },
        ))
    }
}
