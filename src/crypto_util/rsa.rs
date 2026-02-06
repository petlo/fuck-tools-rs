use anyhow::{Error, anyhow};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use rand::rng;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::signature::{RandomizedSigner, SignatureEncoding};
use rsa::traits::PublicKeyParts;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use sha1::Sha1;

pub struct RsaUtil;

impl RsaUtil {
    pub fn gen_rsa_base64() -> Result<(String, String), Error> {
        let mut rng = rand::rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits)?;
        let pub_key = RsaPublicKey::from(&private_key);
        let private_pkcs8_der = private_key.to_pkcs8_der()?;
        let public_key_der = pub_key.to_public_key_der()?;
        let private_key_str = BASE64_STANDARD.encode(private_pkcs8_der.as_bytes());
        let public_key_str = BASE64_STANDARD.encode(public_key_der.as_bytes());
        Ok((private_key_str, public_key_str))
    }

    pub fn rsa_encrypt(content: &str, public_key: Vec<u8>) -> Result<String, Error> {
        let pub_key = RsaPublicKey::from_public_key_der(public_key.as_ref())?;
        let mut rng = rand::rng();

        let key_size = pub_key.size(); // 字节数，2048位 = 256字节
        let max_encrypt_block = key_size - 11; // PKCS#1 v1.5 填充占用11字节

        let data = content.as_bytes();
        let input_len = data.len();
        let mut output = Vec::new();

        let mut offset = 0;
        while offset < input_len {
            let chunk_len = if input_len - offset > max_encrypt_block {
                max_encrypt_block
            } else {
                input_len - offset
            };
            let chunk = &data[offset..offset + chunk_len];
            let encrypted_chunk = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, chunk)?;
            output.extend_from_slice(&encrypted_chunk);
            offset += chunk_len;
        }
        Ok(BASE64_STANDARD.encode(output))
    }

    pub fn rsa_decrypt(content: &str, private_key: Vec<u8>) -> Result<String, Error> {
        let private_key = RsaPrivateKey::from_pkcs8_der(private_key.as_ref())?;

        let key_size = private_key.size();
        let max_decrypt_block = key_size;
        let encrypted_data = BASE64_STANDARD.decode(content)?;
        let input_len = encrypted_data.len();
        if input_len % key_size != 0 {
            return Err(anyhow!(
                "密文长度不正确: {input_len}字节，必须是{key_size}字节的倍数"
            ));
        }

        let mut output = Vec::new();
        let mut offset = 0;
        let mut i = 0;

        while input_len - offset > 0 {
            let chunk_len = if input_len - offset > max_decrypt_block {
                max_decrypt_block
            } else {
                input_len - offset
            };
            let chunk = &encrypted_data[offset..offset + chunk_len];
            let decrypted_chunk = private_key.decrypt(Pkcs1v15Encrypt, chunk)?;
            output.extend_from_slice(&decrypted_chunk);
            i += 1;
            offset = i * max_decrypt_block;
        }
        Ok(String::from_utf8(output)?)
    }

    pub fn sha1_with_rsa(content: &str, private_key: Vec<u8>) -> Result<String, Error> {
        let private_key = RsaPrivateKey::from_pkcs8_der(private_key.as_ref())?;
        let mut rng = rng();
        let signing_key = SigningKey::<Sha1>::new(private_key.clone());
        let signature = signing_key.sign_with_rng(&mut rng, content.as_bytes());
        Ok(BASE64_STANDARD.encode(signature.to_bytes()))
    }
}
