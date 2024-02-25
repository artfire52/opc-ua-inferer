pub struct DataTypeId;
impl DataTypeId {
    pub const NULL: u8 = 0;
    pub const BOOLEAN: u8= 1;
    pub const SBYTE: u8= 2;
    pub const BYTE: u8= 3;
    pub const INT_16: u8= 4;
    pub const UINT_16: u8= 5;
    pub const INT_32: u8= 6;
    pub const UINT_32: u8= 7;
    pub const INT_64: u8= 8;
    pub const UINT_64: u8= 9;
    pub const FLOAT: u8= 10;
    pub const DOUBLE: u8= 11;
    pub const STRING: u8= 12;
    pub const DATETIME: u8= 13;
    pub const GUID: u8= 14;
    pub const BYTESTRING: u8= 15;
    pub const XMLELEMENT: u8= 16;
    pub const NODEID: u8= 17;
    pub const EXPANDED_NODEID: u8= 18;
    pub const STATUS_CODE: u8= 19;
    pub const QUALIFIE_NAME: u8= 20;
    pub const LOCALIZED_TEXT: u8= 21;
    pub const EXTENSION_OBJECT: u8= 22;
    pub const DATAVALUE: u8= 23;
    pub const VARIANT: u8= 24;
    pub const DIAGNOSTIC_INFO: u8= 25;
    pub const ARRAY_DIM_ENCODED: u8= 0b0100_0000;
    pub const ARRAY_VALUE_ENCODED: u8= 0b1000_0000;
    
}