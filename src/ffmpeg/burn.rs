use std::path::Path;

use crate::error::{AppError, Result};
use crate::ffmpeg::Ffmpeg;

/// 使用 FFmpeg `subtitles` 滤镜将 SRT 硬编码到视频画面。
///
/// - 视频：经 `-vf subtitles=...` 重编码（未显式指定视频编码器，由 FFmpeg 默认选择）
/// - 音频：`-c:a copy`，直接复制原音轨
///
/// # Windows 路径
///
/// 在字幕文件所在目录下执行 FFmpeg，滤镜仅使用 SRT **文件名**（相对路径），
/// 避免 `C:/Users/...` 中的冒号被滤镜解析为选项分隔符。
pub fn burn_subtitles(
    ffmpeg: &Ffmpeg,
    input: &Path,
    srt: &Path,
    output: &Path,
) -> Result<()> {
    let srt_dir = srt
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .ok_or_else(|| AppError::InvalidPath(srt.to_path_buf()))?;
    let srt_name = srt
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| AppError::InvalidPath(srt.to_path_buf()))?;

    let input = input
        .canonicalize()
        .unwrap_or_else(|_| input.to_path_buf())
        .to_string_lossy()
        .into_owned();
    let output = output.to_string_lossy().into_owned();
    let filter = format!("subtitles={}", escape_filter_filename(srt_name));

    ffmpeg.run_in(
        Some(srt_dir),
        &[
            "-y",
            "-i",
            &input,
            "-vf",
            &filter,
            "-c:a",
            "copy",
            &output,
        ],
    )?;

    Ok(())
}

/// 转义滤镜参数中的字幕文件名（含空格等时需加引号）。
fn escape_filter_filename(name: &str) -> String {
    if name.contains([' ', ':', '\'', '\\', '[', ',', ';']) {
        format!("'{name}'")
    } else {
        name.to_string()
    }
}
