use crate::utils::tools_crypto::CryptoTools;
use sm_crypto::{sm2, sm3, sm4};

pub struct SmTools;

impl SmTools {
    pub fn sm2_sign(private_key: &str, text: &str) -> String {
        let sign_ctx = sm2::Sign::new(private_key);
        let sign_bytes = sign_ctx.sign_der(text.as_bytes(), true);
        CryptoTools::base64_encode(&sign_bytes)
    }

    pub fn sm3(vec_u8: &[u8]) -> String {
        sm3::sm3_hash(vec_u8)
    }

    pub fn sm4_ecb(key: &[u8], vec_u8: &[u8], is_encrypt: bool) -> Vec<u8> {
        let sm4_ecb = sm4::CryptSM4ECB::new(key);

        if is_encrypt {
            sm4_ecb.encrypt_ecb(vec_u8)
        } else {
            sm4_ecb.decrypt_ecb(vec_u8)
        }
    }

    pub fn sm4_ecb_decrypt(key: &[u8], base64_str: &str) -> Vec<u8> {
        let sm4_ecb = sm4::CryptSM4ECB::new(key);
        sm4_ecb.decrypt_ecb_base64(base64_str)
    }

    pub fn sm4_cbc(key: &[u8], iv: &[u8], vec_u8: &[u8], is_encrypt: bool) -> Vec<u8> {
        let sm4_cbc = sm4::CryptSM4CBC::new(key, iv);

        if is_encrypt {
            sm4_cbc.encrypt_cbc(vec_u8)
        } else {
            sm4_cbc.decrypt_cbc(vec_u8)
        }
    }
}
