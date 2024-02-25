pub struct MessageSecurityMode(u32);
impl MessageSecurityMode {
    pub const INVALID: u32 = 0;
    pub const NONE: u32 = 1;
    pub const SIGN: u32 = 2;
    pub const SIGN_AND_ENCRYPT: u32 = 3;
}
