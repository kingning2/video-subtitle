//! 四步流水线编排：串联 [`crate::ffmpeg`]、[`crate::whisper`]、[`crate::srt`] 模块。

use std::path::{Path, PathBuf};

use crate::cli::Cli;
use crate::error::Result;
use crate::ffmpeg::{burn_subtitles, extract_audio, Ffmpeg};
use crate::srt::write_srt;
use crate::whisper::transcribe;
use crate::whisper::TranscribeOptions;

/// 流水线执行完成后各输出文件的路径。
pub struct PipelineOutput {
    /// 中间 WAV 路径（`{原名}.tmp.wav`）；未加 `--keep-temp` 时可能已被删除。
    pub wav_path: PathBuf,
    /// 生成的 SRT 路径（`{原名}.tmp.srt`）。
    pub srt_path: PathBuf,
    /// 烧录字幕后的视频路径；[`Cli::srt_only`] 时为 `None`。
    pub video_path: Option<PathBuf>,
}

/// 根据 [`Cli`] 配置执行完整字幕流水线。
///
/// ## 步骤
///
/// 1. 解析 FFmpeg 可执行文件
/// 2. 提取 16 kHz 单声道 WAV
/// 3. Whisper 转写为 [`crate::Caption`] 列表
/// 4. 写入 SRT；若未设置 `--srt-only` 则烧录到输出视频
///
/// ## 临时文件
///
/// 默认在输入视频同目录生成 `{stem}.tmp.wav` 与 `{stem}.tmp.srt`。
/// 未指定 `--keep-temp` 时：始终删除 WAV；烧录成功后还会删除 SRT。
pub fn run(cli: &Cli) -> Result<PipelineOutput> {
    let ffmpeg = Ffmpeg::resolve(cli.ffmpeg.as_deref())?;

    let input = &cli.input;
    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let parent = input.parent().unwrap_or_else(|| Path::new("."));

    let wav_path = parent.join(format!("{stem}.tmp.wav"));
    let srt_path = parent.join(format!("{stem}.tmp.srt"));
    let output_video = cli
        .output
        .clone()
        .unwrap_or_else(|| parent.join(format!("{stem}_subtitled.mp4")));

    eprintln!("[1/4] FFmpeg 提取音频 → {}", wav_path.display());
    extract_audio(&ffmpeg, input, &wav_path)?;

    eprintln!("[2/4] Whisper 识别…");
    let captions = transcribe(
        &wav_path,
        &TranscribeOptions {
            model_path: &cli.model,
            language: cli.language.as_deref(),
            translate: cli.translate,
        },
    )?;
    eprintln!("      共 {} 条字幕", captions.len());

    eprintln!("[3/4] 生成 SRT → {}", srt_path.display());
    write_srt(&srt_path, &captions)?;

    let video_path = if cli.srt_only {
        eprintln!("[4/4] 跳过烧录（--srt-only）");
        None
    } else {
        eprintln!("[4/4] FFmpeg 烧录字幕 → {}", output_video.display());
        burn_subtitles(&ffmpeg, input, &srt_path, &output_video)?;
        Some(output_video)
    };

    if !cli.keep_temp {
        let _ = std::fs::remove_file(&wav_path);
        if video_path.is_some() {
            let _ = std::fs::remove_file(&srt_path);
        }
    }

    Ok(PipelineOutput {
        wav_path,
        srt_path,
        video_path,
    })
}
