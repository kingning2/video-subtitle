//! # video-subtitle
//!
//! 视频自动字幕库：串联 FFmpeg 与 Whisper，完成「提取音频 → 语音识别 → 生成 SRT → 烧录字幕」全流程。
//!
//! ## 流水线
//!
//! ```text
//! 输入视频
//!   → ffmpeg::extract_audio   （16 kHz 单声道 WAV）
//!   → whisper::transcribe     （带时间轴的 [`Caption`]）
//!   → srt::write_srt          （SubRip `.srt`）
//!   → ffmpeg::burn_subtitles  （可选，硬字幕输出）
//! ```
//!
//! ## 作为库使用
//!
//! ```no_run
//! use clap::Parser;
//! use video_subtitle::cli::Cli;
//! use video_subtitle::pipeline::run;
//!
//! let cli = Cli::parse();
//! let output = run(&cli)?;
//! # Ok::<(), video_subtitle::AppError>(())
//! ```
//!
//! 命令行入口见 `src/main.rs`。

pub mod cli;
pub mod error;
pub mod ffmpeg;
pub mod pipeline;
pub mod srt;
pub mod types;
pub mod whisper;

pub use error::{AppError, Result};
pub use pipeline::{run, PipelineOutput};
pub use types::Caption;
