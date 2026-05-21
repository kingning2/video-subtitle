# 发布 checklist

## 发布前必改

编辑 `Cargo.toml`：

```toml
authors = ["你的名字 <你的邮箱>"]
repository = "https://github.com/你的用户名/video-subtitle"
homepage = "https://github.com/你的用户名/video-subtitle"
```

若 crate 名 `video-subtitle` 已被占用，可改成例如 `video-subtitle-cli`，并同步修改 `Cargo.toml` 的 `name` 与 README 中的安装命令。

## 首次发布步骤

```powershell
cd D:\Desktop\test

# 1. 初始化 git（cargo publish 建议有版本控制）
git init
git add .
git commit -m "chore: prepare v0.1.0 for crates.io"

# 2. 注册 crates.io 账号，在个人设置里创建 API Token
#    https://crates.io/settings/tokens

# 3. 登录（只需一次）
cargo login

# 4. 检查将要上传的包内容
cargo package --allow-dirty
# 解压查看: target/package/video-subtitle-0.1.0.crate

# 5. 发布（不可撤销版本号，只能 yank）
cargo publish
```

## 安装验证（发布后几分钟生效）

```bash
cargo install video-subtitle
video-subtitle --help
```

## 后续版本

1. 修改 `Cargo.toml` 的 `version`（语义化版本）
2. `git commit` + 打 tag（可选）：`git tag v0.1.1`
3. `cargo publish`

## 常见问题

| 问题 | 处理 |
|------|------|
| crate 名已存在 | 换 `name`，或联系原维护者 |
| `cargo publish` 要求 git | `git init` 并 commit，或了解 `--allow-dirty` 仅用于 package 测试 |
| 用户安装编译失败 | 文档中强调需 CMake、C++、FFmpeg；与本地相同 |
| docs.rs 构建失败 | 查看 https://docs.rs/crate/video-subtitle/builds ，必要时在 `Cargo.toml` 增加 `[package.metadata.docs.rs]` 配置 |
