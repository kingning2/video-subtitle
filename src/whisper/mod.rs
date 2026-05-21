//! Whisper 语音识别：加载 WAV、调用 `whisper-rs` 推理并产出 [`crate::Caption`] 列表。
//!
//! 依赖本地 GGML 模型文件（如 `ggml-base.bin`），无需网络 API。

mod audio;
mod transcribe;

pub use transcribe::{transcribe, TranscribeOptions};
