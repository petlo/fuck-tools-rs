# 构建项目
`cargo build`
`cargo build --release`  # 发布构建

# 运行测试
`cargo test`  # 运行所有测试
`cargo test -- --nocapture`  # 显示输出
`cargo test test_clean_str`  # 运行特定测试

# 检查代码
`cargo check`  # 快速检查编译
`cargo clippy`  # 代码检查
`cargo fmt`  # 格式化代码

# 2. 运行测试
`cargo test`

# 3. 检查代码质量
`cargo clippy`
`cargo fmt --check`

# 4. 生成文档（可选但推荐）
`cargo doc --no-deps --open`

# 1. 首先进行试发布（dry-run）
`cargo publish --dry-run`

# 2. 如果没有错误，正式发布
`cargo publish`