# 发布 checklist

## 发布前检查

- [x] `Cargo.toml` 已填写 `kingning2` 与 GitHub 仓库地址  
- [ ] 代码已推到 https://github.com/kingning2/video-subtitle（见下方「创建 GitHub 仓库」）  
- [ ] （可选）把 `authors` 邮箱改成你的真实邮箱或 GitHub 提供的 noreply 地址  
- [x] crates.io 上 **尚无** `video-subtitle` 同名 crate，可以发布  

若 crate 名被占用，可改成例如 `video-subtitle-cli`，并同步修改 `Cargo.toml` 的 `name` 与 README。

## 创建 GitHub 仓库（当前尚未创建）

1. 打开 https://github.com/new  
2. Repository name：`video-subtitle`  
3. Owner：`kingning2`  
4. **不要**勾选 “Add a README”（本地已有）  
5. 创建后在本机执行：

```powershell
cd D:\Desktop\test
git remote add origin https://github.com/kingning2/video-subtitle.git
git push -u origin master
```

若 GitHub 提示用 `main` 分支：

```powershell
git branch -M main
git push -u origin main
```

## 首次发布步骤

```powershell
cd D:\Desktop\test

# 1. 确保代码已 push 到 GitHub（见上一节）
git remote add origin https://github.com/kingning2/video-subtitle.git
git push -u origin master
# 若默认分支是 main：git branch -M main && git push -u origin main

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
