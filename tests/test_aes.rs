#[cfg(test)]
mod tests {
    use fuck_tools_rs::crypto_util::aes::AesUtil;

    #[test]
    fn test_gen_aes() {
        let data = AesUtil::generate_aes256_key();
        println!("data: {}", data);
    }
}
