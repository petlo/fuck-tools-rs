use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use hex::FromHexError;
use md5::{Digest, Md5};
use ring::digest;

pub struct CryptoTools;

impl CryptoTools {
    pub fn md5_hex(str: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(str);
        hex::encode(hasher.finalize())
    }

    pub fn hex_encode(str: &[u8]) -> String {
        hex::encode(str)
    }

    pub fn hex_decode(str: &str) -> Result<Vec<u8>, FromHexError> {
        hex::decode(str)
    }

    pub fn base64_encode(bytes: &[u8]) -> String {
        BASE64_STANDARD.encode(bytes)
    }

    pub fn base64_decode(bytes: &[u8]) -> Option<Vec<u8>> {
        BASE64_STANDARD.decode(bytes).ok()
    }

    pub fn sha_1(str: &str) -> String {
        let hash = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, str.as_bytes());
        hex::encode(hash.as_ref())
    }

    pub fn sha_256(str: &str) -> String {
        hex::encode(digest::digest(&digest::SHA256, str.as_bytes()))
    }

    pub fn hmac_sha256_base64(security_key: &str, content: &str) -> String {
        let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, security_key.as_bytes());
        let tag = ring::hmac::sign(&key, content.as_bytes());
        Self::base64_encode(tag.as_ref())
    }
}
