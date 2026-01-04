use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use md5::{Digest, Md5};
use ring::digest;

pub struct CryptoTools;

impl CryptoTools {

    pub fn md5(str: &str) -> String {
        let mut hasher = Md5::new();
        hasher.update(str);
        hex::encode(hasher.finalize())
    }

    pub fn base64_encode(str: &str) -> String {
        BASE64_STANDARD.encode(str.as_bytes())
    }

    pub fn base64_decode(str: &str) -> Option<Vec<u8>> {
        BASE64_STANDARD.decode(str.as_bytes()).ok()
    }

    pub fn sha_1(str: &str) -> String {
        let hash = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, str.as_bytes());
        hex::encode(hash.as_ref())
    }

    pub fn sha_256(str: &str) -> String {
        hex::encode(digest::digest(&digest::SHA256, str.as_bytes()))
    }
}