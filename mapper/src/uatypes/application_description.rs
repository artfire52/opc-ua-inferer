use derive_macro::{Deserialize, Serialize};

use crate::{Deserialize, Serialize, MapperResult};

use super::{localized_text::LocalizedText, string::UaString};

#[derive(Debug, PartialEq, Deserialize, Serialize,Clone)]
pub (crate) struct ApplicationDescription {
    pub(crate) application_uri: UaString,
    pub(crate) product_uri: UaString,
    pub(crate) application_name: LocalizedText,
    pub(crate) application_type: u32,
    pub(crate) gateway_server_uri: UaString,
    pub(crate) discovery_policy_uri: UaString,
    pub(crate) discovery_urls: Vec::<UaString>,
}

pub struct ApplicationType;
impl ApplicationType {
    pub const SERVER: u32 = 0;
    pub const CLIENT: u32 = 1;
    pub const CLIENTANDSERVER: u32 = 2;
    pub const DISCOVERYSEREVR: u32 = 3;
}
