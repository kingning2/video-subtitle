//! 命令行参数定义（基于 [`clap`]）。

use std::path::PathBuf;

use clap::Parser;

/// 命令行配置，由 [`clap::Parser`] 从进程参数解析。
///
/// # 示例
///
/// ```bash
/// video-subtitle -i demo.mp4 -m ggml-base.bin -l zh
/// video-subtitle -i demo.mp4 -m ggml-small.bin --srt-only --keep-temp
/// ```
#[derive(Debug, Parser)]
#[command(
    name = "video-subtitle",
    about = "FFmpeg 提取音频 → Whisper 识别 → SRT → FFmpeg 烧录字幕",
    version
)]
pub struct Cli {
    /// 输入视频文件路径。
    #[arg(short, long)]
    pub input: PathBuf,

    /// 烧录字幕后的输出视频路径。
    ///
    /// 未指定时默认为输入文件同目录下的 `{原名}_subtitled.mp4`。
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Whisper GGML 模型文件路径（如 `ggml-base.bin`）。
    #[arg(short, long)]
    pub model: PathBuf,

    /// 识别语言 ISO 代码，例如 `zh`、`en`。
    ///
    /// 传 `auto` 或省略则由 Whisper 自动检测。
    #[arg(short, long)]
    pub language: Option<String>,

    /// 启用 Whisper 翻译模式：将非英语内容译为英语。
    #[arg(long)]
    pub translate: bool,

    /// 仅生成 SRT，不执行第四步字幕烧录。
    #[arg(long)]
    pub srt_only: bool,

    /// 流程结束后保留临时 `.tmp.wav` / `.tmp.srt` 文件。
    #[arg(long)]
    pub keep_temp: bool,

    /// `ffmpeg` 可执行文件路径；省略则在 PATH 中查找 `ffmpeg` / `ffmpeg.exe`。
    #[arg(long)]
    pub ffmpeg: Option<PathBuf>,
}
