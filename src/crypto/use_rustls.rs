use aes::cipher::{KeyInit, BlockEncryptMut, BlockSizeUser, block_padding::{Pkcs7, UnpadError}, BlockDecryptMut, KeyIvInit};
use rand::{RngCore, CryptoRng, rngs::ThreadRng};
use ring::signature::{RsaKeyPair, RsaSubjectPublicKey};
use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt, rand_core::CryptoRngCore};
use rustls::internal::msgs::codec::Codec;

use super::key::EAPI_KEY;

pub(crate) fn md5_hex(pt: &[u8]) -> String {
    hex::encode(md5::compute(pt).to_vec())
}

#[allow(unused)]
pub fn eapi_decrypt(ct: &[u8]) -> Result<Vec<u8>, UnpadError> {
    aes_128_ecb_decrypt(ct, EAPI_KEY.as_bytes())
}

pub fn aes_128_ecb(pt: &[u8], key: &[u8]) -> Vec<u8> {
    let mut res = vec![0; pt.len() +  aes::Aes128::block_size()];
    let r = ecb::Encryptor::<aes::Aes128>::new(key.into()).encrypt_padded_b2b_mut::<Pkcs7>(pt, &mut res).unwrap();
    let len = r.len();
    drop(r);
    res.truncate(len);
    res
}

fn aes_128_ecb_decrypt(ct: &[u8], key: &[u8]) -> Result<Vec<u8>, UnpadError> {
    let mut res = vec![0; ct.len() + aes::Aes128::block_size()];
    let r = ecb::Decryptor::<aes::Aes128>::new(key.into()).decrypt_padded_b2b_mut::<Pkcs7>(ct, &mut res)?;
    let len = r.len();
    drop(r);
    res.truncate(len);
    Ok(res)
}

pub fn aes_128_cbc(pt: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut res = vec![0; pt.len() + aes::Aes128::block_size()];
    let r = cbc::Encryptor::<aes::Aes128>::new(key.into(), iv.into()).encrypt_padded_b2b_mut::<Pkcs7>(pt, &mut res).unwrap();
    let len = r.len();
    drop(r);
    res.truncate(len);
    res
}

struct PRng {
    rng: ThreadRng 
}

impl CryptoRng for PRng {
    
}
impl RngCore for PRng {
    fn next_u32(&mut self) -> u32 {
        65537
    }

    fn next_u64(&mut self) -> u64 {
        65537
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

pub fn rsa(pt: &[u8], key: &str) -> Vec<u8> {
    let rng = rand::thread_rng();

    let rsa = RsaPublicKey::from_public_key_pem(key).unwrap();

    let ct = rsa.encrypt(&mut PRng {rng}, Pkcs1v15Encrypt, &pt).unwrap();
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
        println!("{}", hex::encode(&ct));
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
        println!("{}", hex::encode(&ct));
        assert!(hex::encode(ct).ends_with("4413"));
    }
}

