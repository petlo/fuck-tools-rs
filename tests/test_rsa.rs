#[cfg(test)]
mod tests {
    use fuck_tools_rs::crypto_util::rsa::RsaUtil;

    #[test]
    fn test_gen_rsa() {
        let (private_key, public_key) = RsaUtil::gen_rsa_base64().unwrap();
        println!("private_key: {}", private_key);
        println!("public_key: {}", public_key);
    }
}
