use aes::Aes128;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray};
use anyhow::{Error, bail};
use hex;

pub struct AesTools;

impl AesTools {
    pub fn aes_ecb_encrypt(text: &str, key: &[u8]) -> Result<String, Error> {
        // 确保 key 是 16 字节
        if key.len() != 16 {
            bail!("AES encryption key must be 16 bytes");
        }

        // 将 key 转换为 GenericArray
        let key_bytes = GenericArray::clone_from_slice(key);

        // 创建 AES 加密器
        let cipher = Aes128::new(&key_bytes);

        // 确保文本长度是 16 字节的倍数（AES 块大小）
        let padded_text = Self::pad_pkcs7(text.as_bytes(), 16);

        // 加密每个块
        let mut encrypted = Vec::new();
        for chunk in padded_text.chunks(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.encrypt_block(&mut block);
            encrypted.extend_from_slice(&block);
        }

        // 返回十六进制字符串
        Ok(hex::encode(encrypted))
    }

    pub fn aes_ecb_decrypt(ciphertext_hex: &str, key: &[u8]) -> Result<String, Error> {
        // 确保 key 是 16 字节
        if key.len() != 16 {
            bail!("AES encryption key must be 16 bytes");
        }

        // 将十六进制字符串解码为字节
        let ciphertext = match hex::decode(ciphertext_hex) {
            Ok(data) => data,
            Err(e) => bail!(e),
        };

        // 确保密文长度是 16 字节的倍数
        if ciphertext.len() % 16 != 0 {
            bail!("AES decryption key must be 16 bytes");
        }

        // 将 key 转换为 GenericArray
        let key_bytes = GenericArray::clone_from_slice(key);
        let cipher = Aes128::new(&key_bytes);

        // 解密每个块
        let mut decrypted = Vec::new();
        for chunk in ciphertext.chunks(16) {
            let mut block = GenericArray::clone_from_slice(chunk);
            cipher.decrypt_block(&mut block);
            decrypted.extend_from_slice(&block);
        }

        // 移除 PKCS#7 填充并转换为字符串
        let unpadded = Self::unpad_pkcs7(&decrypted);
        Ok(String::from_utf8_lossy(&unpadded).to_string())
    }

    // PKCS#7 填充
    fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
        let padding = block_size - (data.len() % block_size);
        let mut padded = data.to_vec();
        padded.extend(vec![padding as u8; padding]);
        padded
    }

    // 移除 PKCS#7 填充
    fn unpad_pkcs7(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let padding = *data.last().unwrap() as usize;

        // 验证填充是否有效
        if padding > data.len() || padding == 0 {
            return data.to_vec(); // 返回原始数据，可能没有填充
        }

        // 检查所有填充字节是否正确
        for i in (data.len() - padding)..data.len() {
            if data[i] as usize != padding {
                return data.to_vec(); // 返回原始数据，可能没有填充
            }
        }

        data[..data.len() - padding].to_vec()
    }
}
