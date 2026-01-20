# Fuck Tools Rs

[![Crates.io](https://img.shields.io/crates/v/fuck-tools-rs.svg)](https://crates.io/crates/fuck-tools-rs)
[![Docs.rs](https://docs.rs/fuck-tools-rs/badge.svg)](https://docs.rs/fuck-tools-rs)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()
[![Rust](https://img.shields.io/badge/rust-1.60+-orange.svg)](https://www.rust-lang.org)

一个功能丰富的 Rust 工具库，提供日常开发中常用的工具函数。

> **注意**: 本项目主要用于学习 Rust 和简化常见操作，API 设计以易用性为优先考虑。

## ✨ 特性

### ⏰ 时间工具 (Time Utilities)
- **时间格式化**: 灵活的时间格式转换和解析
- **时间计算**: 时间加减、时间差计算、相对时间
- **时间戳操作**: 秒级/毫秒级时间戳获取、转换和解析
- **时区处理**: 本地时间与 UTC 时间互转

### 🆔 标识符工具 (ID Utilities)
- **UUID 生成**: 支持 UUID v4、v7 版本生成
- **Snowflake ID**: 分布式雪花算法 ID 生成器
- **ID 解析验证**: UUID 字符串解析和格式验证
- **类型转换**: UUID 与字符串的相互转换

### 🔒 加密工具 (Cryptography Tools)
- **加解密工具**:
    - **SHA-256**: 256 位安全哈希算法
    - **MD5**: 128 位哈希算法（注意：仅适用于兼容场景）
    - **SM3**: 国密 SM3 哈希算法
    - **AES**: AES 工具
    - **SM4**: SM4 工具
    - **RSA**: RSA 加密/解密 工具
    - **SM2**: 国密 SM2 签名 工具
    - **Base64**: Base64 编码解码
    - **十六进制**: HEX 编码解码

### 📋 数据处理 (Data Processing)
- **JSON 处理**:
    - JSON 字典按 Key 排序
    - JSON 格式化美化
    - JSON 安全解析
- **字符串工具**:
    - 常用字符串操作
    - 字符编码转换
- **数值转换**: 安全的类型转换和格式化

## 📦 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
fuck-tools-rs = "0.0.4"