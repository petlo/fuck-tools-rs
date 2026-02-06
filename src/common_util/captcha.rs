use captcha_rs::CaptchaBuilder;

pub struct CaptchaUtil;

/// 验证码构建器
///
/// 用于配置和生成验证码的参数
#[derive(Debug, Clone)]
pub struct CaptchaInfo {
    /// 验证码字符数量
    ///
    /// 指定验证码中显示的字符数量。
    /// - 建议范围：4-8个字符
    /// - 默认值：4
    /// - 太少的字符会降低安全性
    /// - 太多的字符会影响用户体验
    pub char_num: usize,

    /// 图片宽度（像素）
    ///
    /// 验证码图片的宽度，单位为像素。
    /// - 建议范围：120-320像素
    /// - 默认值：160
    /// - 根据显示设备的分辨率调整
    pub width: u32,

    /// 图片高度（像素）
    ///
    /// 验证码图片的高度，单位为像素。
    /// - 应与宽度保持适当比例（通常为1:2.5到1:3）
    pub height: u32,

    /// 深色模式
    ///
    /// 控制验证码的主题颜色。
    /// - `true`: 深色模式，使用深色背景（如黑色、深灰色）和浅色文字
    /// - `false`: 浅色模式，使用浅色背景（如白色、浅灰色）和深色文字（默认）
    pub dark_mode: bool,

    /// 复杂度级别
    ///
    /// 控制验证码的干扰元素复杂程度 1-10。
    pub complexity: u32,

    /// 图片压缩质量
    ///
    /// 控制输出图片的压缩质量（仅对JPEG格式有效，PNG为无损压缩）1-99。
    pub compression: u8,
}

impl Default for CaptchaInfo {
    fn default() -> Self {
        Self {
            char_num: 4,
            width: 220,
            height: 220,
            dark_mode: false,
            complexity: 1,
            compression: 40,
        }
    }
}

impl CaptchaUtil {
    pub fn gen_captcha_code(captcha_info: CaptchaInfo) -> (String, String) {
        let captcha = CaptchaBuilder::new()
            .length(captcha_info.char_num)
            .width(captcha_info.width)
            .height(captcha_info.height)
            .dark_mode(captcha_info.dark_mode)
            .complexity(captcha_info.complexity)
            .compression(captcha_info.compression)
            .build();
        let captcha_base64 = captcha.to_base64();
        (captcha.text, captcha_base64)
    }
}
