#[cfg(test)]
mod tests {
    use fuck_tools_rs::utils::tools_id::IdTools;

    #[test]
    fn test_snowflake_id() {
        println!("generated id1: {}", IdTools::gen_snowflake_id());
    }

    #[test]
    fn test_uuid_v4() {
        let id1 = IdTools::gen_uuid_v4();
        let id2 = IdTools::gen_uuid_v4_simple();
        println!("generated id1: {}", id1);
        println!("generated id2: {}", id2);
    }

    #[test]
    fn test_uuid_v7() {
        let id1 = IdTools::gen_uuid_v7();
        let id2 = IdTools::gen_uuid_v7_simple();
        println!("generated id1: {}", id1);
        println!("generated id2: {}", id2);
    }

    #[test]
    fn test_prefix() {
        let id1 = IdTools::gen_uuid_v4_with_prefix("v4-");
        let id2 = IdTools::gen_uuid_v7_with_prefix("v7-");
        let id3 = IdTools::gen_snowflake_id_with_prefix("snowflake-");
        println!("generated id1: {}", id1);
        println!("generated id2: {}", id2);
        println!("generated id3: {}", id3);
    }

    #[test]
    fn test_multiple() {
        let id1s = IdTools::gen_multiple_uuid_v4(2);
        let id2s = IdTools::gen_multiple_snowflake_ids(2);
        println!("generated id1: {:?}", id1s);
        println!("generated id2: {:?}", id2s);
    }

    #[test]
    fn test_incremental() {
        let id = IdTools::gen_incremental_id(&mut 2);
        println!("generated id1: {:?}", id);
    }

    #[test]
    fn test_is_valid_uuid() {
        let data1 = IdTools::is_valid_uuid("uuid");
        let uuid_str = IdTools::gen_uuid_v4().to_string();
        let data2 = IdTools::is_valid_uuid(uuid_str.as_str());
        println!("data1: {}", data1);
        println!("data2: {}", data2);
    }

    #[test]
    fn test_parse_uuid() {
        let uuid_str = IdTools::gen_uuid_v4().to_string();
        let data = IdTools::parse_uuid(uuid_str.as_str());
        println!("data: {:?}", data);
    }
}
