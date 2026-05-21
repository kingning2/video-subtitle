//! `video-subtitle` 命令行入口。
//!
//! 解析 [`video_subtitle::cli::Cli`]，校验输入路径与 FFmpeg 可用性后，
//! 调用 [`video_subtitle::pipeline::run`] 执行流水线。

use clap::Parser;
use video_subtitle::cli::Cli;
use video_subtitle::ffmpeg::check_ffmpeg;
use video_subtitle::pipeline::run;

fn main() {
    if let Err(e) = main_inner() {
        eprintln!("错误: {e}");
        std::process::exit(1);
    }
}

/// 实际业务逻辑；错误向上返回供 `main` 统一打印并设置退出码。
fn main_inner() -> video_subtitle::Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        return Err(video_subtitle::AppError::InvalidPath(cli.input.clone()));
    }
    if !cli.model.exists() {
        return Err(video_subtitle::AppError::InvalidPath(cli.model.clone()));
    }

    check_ffmpeg(cli.ffmpeg.as_deref())?;

    let out = run(&cli)?;

    eprintln!("\n完成。");
    eprintln!("  SRT: {}", out.srt_path.display());
    if let Some(v) = &out.video_path {
        eprintln!("  视频: {}", v.display());
    }
    if cli.keep_temp {
        eprintln!("  临时 WAV: {}", out.wav_path.display());
    }

    Ok(())
}
