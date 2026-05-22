# 发布 checklist

## 当前状态

| 渠道 | 状态 |
|------|------|
| [crates.io](https://crates.io/crates/video-subtitle) | 已发布 `video-subtitle` v0.1.0 |
| [docs.rs](https://docs.rs/video-subtitle) | 已随 crate 构建 |
| [GitHub](https://github.com/kingning2/video-subtitle) | 需首次 push（见下方命令） |
| [xiaoman 资产库](https://kingning2.github.io/xiaoman-projects/) | push + Secrets 后运行注册 workflow |

## 首次推到 GitHub

仓库已创建后，在本机执行（只需做一次 `remote add`）：

```powershell
cd D:\Desktop\test

# 若尚未添加远程
git remote add origin https://github.com/kingning2/video-subtitle.git
# 若已存在但地址不对：git remote set-url origin https://github.com/kingning2/video-subtitle.git

git add .github .gitignore PUBLISHING.md
git commit -m "ci: add GitHub Actions (CI, Pages, portfolio register)"

# 默认分支 master
git push -u origin master
```

若 GitHub 空仓库默认分支是 `main`，二选一：

```powershell
# A：保持本地 master，推 master
git push -u origin master

# B：改名为 main 再推
git branch -M main
git push -u origin main
```

推送后打开仓库 **Settings → Pages → Build and deployment**，将 **Source** 设为 **GitHub Actions**（否则 Pages 部署 workflow 不会出站点）。

## GitHub Actions

| Workflow | 文件 | 作用 |
|----------|------|------|
| **CI** | `.github/workflows/ci.yml` | `cargo check` / `clippy` |
| **Deploy to GitHub Pages** | `.github/workflows/pages.yml` | 发布 `.github/pages` 落地页 |
| **Register to portfolio** | `.github/workflows/register-to-portfolio.yml` | 同步到 xiaoman-projects 资产库 |

### 资产库 Secrets（勿提交 git）

在本仓库 **Settings → Secrets and variables → Actions** 添加：

| Secret | 说明 |
|--------|------|
| `PORTFOLIO_PAT` | 对 `kingning2/xiaoman-projects` 有 Contents 写权限的 PAT |
| `PORTFOLIO_REGISTER_KEY` | 与资产库相同的注册码 |

配置完成后：

1. 等 **Deploy to GitHub Pages** 成功（push 后自动跑），或  
2. **Actions → Register to portfolio → Run workflow** 手动注册  

然后在 [xiaoman-projects Actions](https://github.com/kingning2/xiaoman-projects/actions) 确认 **Sync asset to projects.json** 成功。

## crates.io 发布（已完成 v0.1.0）

后续新版本：

```powershell
cd D:\Desktop\test
# 1. 改 Cargo.toml version，commit + push
git push origin master   # 或 main

# 2. 发布（不可撤销版本号，只能 yank）
cargo publish
```

首次发布若尚未做过：

```powershell
cargo login
cargo package --allow-dirty
cargo publish
```

安装验证：

```bash
cargo install video-subtitle
video-subtitle --help
```

## 常见问题

| 问题 | 处理 |
|------|------|
| `git push` 认证失败 | 使用 HTTPS + PAT，或配置 SSH `git@github.com:kingning2/video-subtitle.git` |
| Pages 无站点 | Settings → Pages → Source 选 **GitHub Actions** |
| register 不触发 | 确认 Pages workflow 名称为 **Deploy to GitHub Pages**；或手动 Run register workflow |
| Invalid registerKey | 本仓 `PORTFOLIO_REGISTER_KEY` 与 xiaoman-projects 不一致 |
| CI 编译 whisper-rs 慢 | 正常；需 cmake 与 C++ 工具链 |
| docs.rs 构建失败 | 查看 https://docs.rs/crate/video-subtitle/builds |
