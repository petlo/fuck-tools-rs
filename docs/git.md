<类型>[可选 范围]: <描述>

[可选 正文]

[可选 脚注]
feat: 新功能
fix: 修复 bug
docs: 文档更新
style: 代码格式调整（不影响代码运行）
refactor: 重构（既不是新功能也不是修复 bug）
perf: 性能优化
test: 测试相关
chore: 构建过程或辅助工具变动
ci: CI 配置相关

[示例1]
feat(auth): 实现 JWT 令牌认证

- 添加 JWT 生成和验证中间件
- 实现用户登录接口
- 添加权限验证装饰器

[示例2]
git commit -m "feat(menu): 新增菜单管理功能"
git commit -m "fix(auth): 修复登录认证问题"
git commit -m "docs: 更新项目文档"


Closes #123