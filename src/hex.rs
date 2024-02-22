use openssl::hash::{hash, MessageDigest};

pub fn md5_hex(pt: &[u8]) -> String {
    let digest = hash(MessageDigest::md5(), pt).unwrap();
    hex::encode(digest)
}
