#[cfg(test)]
mod tests {
    use fuck_tools_rs::crypto_util::password::PasswordUtils;

    #[test]
    fn test_noise() {
        let password1 = "admin123";
        // let password2 = "$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE8ByOhJIrdAu2";
        // let data = PasswordUtils::verify_password(password1, password2).unwrap();
        // println!("{}", data);

        let data = PasswordUtils::hash_password(password1).unwrap();
        println!("data: {:?}", data);
    }
}
