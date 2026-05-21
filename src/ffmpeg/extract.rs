use std::path::Path;

use crate::error::Result;
use crate::ffmpeg::Ffmpeg;

/// 从视频中提取 Whisper 所需的 PCM WAV。
///
/// 等价 FFmpeg 参数概览：
///
/// ```text
/// -vn -acodec pcm_s16le -ar 16000 -ac 1
/// ```
///
/// - 丢弃视频轨（`-vn`）
/// - 16 kHz、单声道、16-bit 有符号小端 PCM
///
/// # 错误
///
/// 输入文件无效或 FFmpeg 执行失败时返回 [`crate::AppError`]。
pub fn extract_audio(ffmpeg: &Ffmpeg, input: &Path, output_wav: &Path) -> Result<()> {
    let input = input.to_string_lossy();
    let output = output_wav.to_string_lossy();

    ffmpeg.run(&[
        "-y",
        "-i",
        &input,
        "-vn",
        "-acodec",
        "pcm_s16le",
        "-ar",
        "16000",
        "-ac",
        "1",
        &output,
    ])?;

    Ok(())
}
