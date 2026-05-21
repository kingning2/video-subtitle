# video-subtitle

[![crates.io](https://img.shields.io/crates/v/video-subtitle.svg)](https://crates.io/crates/video-subtitle)
[![docs.rs](https://docs.rs/video-subtitle/badge.svg)](https://docs.rs/video-subtitle)

视频自动字幕：**FFmpeg 提取音频 → Whisper 识别 → SRT → FFmpeg 烧录硬字幕**。

同时提供 **CLI 工具** 与 **Rust 库**（`video_subtitle`）。

## 安装

### 从 crates.io（需先配置 Rust 与系统依赖）

```bash
cargo install video-subtitle
```

### 从源码

```bash
git clone https://github.com/kingning2/video-subtitle
cd video-subtitle
cargo install --path .
```

## 系统依赖

| 依赖 | 说明 |
|------|------|
| **FFmpeg** | 在 PATH 中可用，或运行时 `--ffmpeg` 指定 |
| **Whisper GGML 模型** | 如 [ggml-small.bin](https://huggingface.co/ggerganov/whisper.cpp/tree/main) |
| **CMake + C++ 编译器** | 编译 `whisper-rs` 时需要（Windows 建议 VS Build Tools） |

中文内容推荐使用 **`ggml-small.bin`** 或更大模型，`-l zh`。

## 用法

```bash
video-subtitle -i input.mp4 -m /path/to/ggml-small.bin -l zh
```

| 参数 | 说明 |
|------|------|
| `-i, --input` | 输入视频（必填） |
| `-m, --model` | Whisper GGML 模型路径（必填） |
| `-o, --output` | 输出视频（默认 `{原名}_subtitled.mp4`） |
| `-l, --language` | 语言，如 `zh`、`en`；`auto` 为自动检测 |
| `--translate` | Whisper 翻译为英语（做中文字幕时不要加） |
| `--srt-only` | 只生成 SRT，不烧录 |
| `--keep-temp` | 保留临时 `.tmp.wav` / `.tmp.srt` |
| `--ffmpeg` | 指定 ffmpeg 可执行文件 |

仅生成字幕：

```bash
video-subtitle -i demo.mp4 -m ggml-small.bin -l zh --srt-only --keep-temp
```

## 作为库使用

```rust
use clap::Parser;
use video_subtitle::cli::Cli;
use video_subtitle::pipeline::run;

let cli = Cli::parse();
let output = run(&cli)?;
println!("SRT: {}", output.srt_path.display());
```

模块说明见 [docs.rs](https://docs.rs/video-subtitle)。

## 发布到 crates.io（维护者）

1. 在 [crates.io](https://crates.io) 注册并获取 API Token  
2. 登录并发布（详见 [PUBLISHING.md](PUBLISHING.md)）：

```bash
cargo login
cargo publish
```

首次发布前本地检查：

```bash
cargo package --allow-dirty
cargo install --path . -f
```

> **注意**：crate 依赖 `whisper-rs`，用户 `cargo install` 时会在本机编译 whisper.cpp，与直接 `cargo build` 相同。

## License

MIT — 见 [LICENSE](LICENSE).
