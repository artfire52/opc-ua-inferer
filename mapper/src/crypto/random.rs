use openssl::rand;

use crate::uatypes::byte_string::ByteString;
/// Fills the slice with cryptographically strong pseudo-random bytes
pub fn bytes(bytes: &mut [u8]) {
    let _ = rand::rand_bytes(bytes);
}


/// Create a byte string with a number of random characters. Can be used to create a nonce or
/// a similar reason.
pub fn byte_string(number_of_bytes: usize) -> ByteString {
    let mut data = vec![0u8; number_of_bytes];
    bytes(&mut data);
    ByteString::from(data)
}
