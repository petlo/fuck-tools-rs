pub struct ClearTools;

impl ClearTools {
    /// 清理字符串：移除双引号并修剪空格
    pub fn clean_str(content: String) -> String {
        content
            .replace("\"", "")
            .replace("\n", "")
            .trim()
            .to_string()
    }

    /// 移除所有空白字符（空格、制表符、换行等）
    pub fn remove_whitespace(content: &str) -> String {
        content.chars().filter(|c| !c.is_whitespace()).collect()
    }

    /// 清理HTML标签
    pub fn strip_html_tags(content: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for c in content.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ => {
                    if !in_tag {
                        result.push(c);
                    }
                }
            }
        }
        result
    }

    /// 移除控制字符和非打印字符
    pub fn remove_control_chars(content: String) -> String {
        content.chars().filter(|c| !c.is_control()).collect()
    }

    /// 规范化空格：将多个连续空格替换为单个空格
    pub fn normalize_whitespace(content: &str) -> String {
        let mut result = String::new();
        let mut last_was_whitespace = false;

        for c in content.chars() {
            if c.is_whitespace() {
                if !last_was_whitespace {
                    result.push(' ');
                    last_was_whitespace = true;
                }
            } else {
                result.push(c);
                last_was_whitespace = false;
            }
        }
        result.trim().to_string()
    }
}
