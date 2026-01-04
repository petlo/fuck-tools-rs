#[cfg(test)]
mod tests {
    use fuck_tools_rs::utils::tools_crypto::CryptoTools;

    #[test]
    fn test_sha_1() {
        println!("{}", CryptoTools::sha_1("Hello world"));
    }

    #[test]
    fn test_sha_256() {
        println!("{}", CryptoTools::sha_256("Hello world"));
    }
}