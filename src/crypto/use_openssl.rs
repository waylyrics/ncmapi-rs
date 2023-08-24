use openssl::hash::{MessageDigest, hash};

use openssl::{
    error::ErrorStack,
    rsa::{Padding, Rsa},
    symm::{decrypt, encrypt, Cipher},
};

use super::key::EAPI_KEY;


pub(crate) fn md5_hex(pt: &[u8]) -> String {
    hex::encode(hash(MessageDigest::md5(), pt).unwrap())
}

#[allow(unused)]
pub(super) fn eapi_decrypt(ct: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    aes_128_ecb_decrypt(ct, EAPI_KEY.as_bytes())
}


pub(super) fn aes_128_ecb(pt: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key, None, pt).unwrap()
}

fn aes_128_ecb_decrypt(ct: &[u8], key: &[u8]) -> Result<Vec<u8>, ErrorStack> {
    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key, None, ct)
}

pub(super) fn aes_128_cbc(pt: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_cbc();
    encrypt(cipher, key, Some(iv), pt).unwrap()
}

pub(super) fn rsa(pt: &[u8], key: &str) -> Vec<u8> {
    let rsa = Rsa::public_key_from_pem(key.as_bytes()).unwrap();
    let prefix = vec![0u8; 128 - pt.len()];
    let pt = [&prefix[..], pt].concat();

    let mut ct = vec![0; rsa.size() as usize];
    rsa.public_encrypt(&pt, &mut ct, Padding::NONE).unwrap();
    ct
}

#[cfg(test)]
mod tests {
    use crate::crypto::key::{PRESET_KEY, IV, EAPI_KEY, PUBLIC_KEY};
    use super::{aes_128_cbc, aes_128_ecb_decrypt, aes_128_ecb, rsa};


    #[test]
    fn test_aes_cbc() {
        let pt = "plain text";
        let ct = aes_128_cbc(pt.as_bytes(), PRESET_KEY.as_bytes(), IV.as_bytes());
        assert!(hex::encode(ct).ends_with("baf0"))
    }

    
    #[test]
    fn test_aes_128_ecb() {
        let pt = "plain text";
        let ct = aes_128_ecb(pt.as_bytes(), EAPI_KEY.as_bytes());
        let _pt = aes_128_ecb_decrypt(&ct, EAPI_KEY.as_bytes());
        assert!(_pt.is_ok());

        if let Ok(decrypted) = _pt {
            assert_eq!(&decrypted, pt.as_bytes());
        }
    }

    #[test]
    fn test_rsa() {
        let ct = rsa(PRESET_KEY.as_bytes(), PUBLIC_KEY);
        assert!(hex::encode(ct).ends_with("4413"));
    }
}