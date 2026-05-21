//! 库内统一错误类型与 [`Result`] 别名。

use std::path::PathBuf;
use thiserror::Error;

/// 本库所有可恢复错误的 [`Result`] 别名。
pub type Result<T> = std::result::Result<T, AppError>;

/// 流水线各阶段可能产生的错误。
#[derive(Debug, Error)]
pub enum AppError {
    /// 系统 PATH 中未找到 `ffmpeg`，且未通过 CLI `--ffmpeg` 指定有效路径。
    #[error("未找到 ffmpeg，请安装并加入 PATH，或通过 --ffmpeg 指定路径")]
    FfmpegNotFound,

    /// FFmpeg 子进程以非零状态退出。
    #[error("ffmpeg 执行失败 (退出码 {code:?}): {stderr}")]
    FfmpegFailed {
        /// 进程退出码；被信号终止时可能为 `None`。
        code: Option<i32>,
        /// FFmpeg 标准错误输出（已去除首尾空白）。
        stderr: String,
    },

    /// 文件读写等标准库 IO 错误。
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// `whisper-rs` / whisper.cpp 推理或模型加载失败。
    #[error("Whisper 错误: {0}")]
    Whisper(#[from] whisper_rs::WhisperError),

    /// WAV 格式不符合要求，或样本读取失败。
    #[error("音频读取错误: {0}")]
    Audio(String),

    /// 输入视频、模型路径等不存在或不可作为 UTF-8 路径使用。
    #[error("路径无效: {0}")]
    InvalidPath(PathBuf),

    /// 其它未分类错误（预留扩展）。
    #[error("{0}")]
    Other(String),
}
