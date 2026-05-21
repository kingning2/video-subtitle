use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{AppError, Result};

/// 已解析的 FFmpeg 可执行文件句柄。
///
/// 通过 [`Ffmpeg::resolve`] 或 [`check_ffmpeg`] 获得，后续 [`extract_audio`](super::extract_audio)
/// 与 [`burn_subtitles`](super::burn_subtitles) 均复用同一实例。
#[derive(Clone, Debug)]
pub struct Ffmpeg {
    executable: PathBuf,
}

impl Ffmpeg {
    /// 解析 FFmpeg 路径。
    ///
    /// - `explicit` 为 `Some` 时使用给定路径（须存在）
    /// - 否则在 PATH 中探测 `ffmpeg` / `ffmpeg.exe`
    pub fn resolve(explicit: Option<&Path>) -> Result<Self> {
        if let Some(path) = explicit {
            let path = path.to_path_buf();
            if !path.exists() {
                return Err(AppError::InvalidPath(path));
            }
            return Ok(Self { executable: path });
        }

        which_ffmpeg().map(|executable| Self { executable })
    }

    /// 已解析的可执行文件路径。
    pub fn path(&self) -> &Path {
        &self.executable
    }

    /// 同步执行 FFmpeg 命令；非零退出码时返回 [`AppError::FfmpegFailed`]。
    ///
    /// `args` 为传给 FFmpeg 的参数列表（不含程序名本身）。
    pub fn run(&self, args: &[&str]) -> Result<std::process::Output> {
        self.run_in(None, args)
    }

    /// 在可选工作目录下执行 FFmpeg（用于 `subtitles=` 使用相对路径，规避 Windows 盘符转义问题）。
    pub fn run_in(&self, cwd: Option<&Path>, args: &[&str]) -> Result<std::process::Output> {
        let mut cmd = Command::new(self.path());
        if let Some(cwd) = cwd {
            cmd.current_dir(cwd);
        }
        let output = cmd.args(args).output()?;

        if output.status.success() {
            Ok(output)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(AppError::FfmpegFailed {
                code: output.status.code(),
                stderr,
            })
        }
    }
}

/// 探测 FFmpeg 是否可用，等价于 [`Ffmpeg::resolve`] 后丢弃实例。
pub fn check_ffmpeg(explicit: Option<&Path>) -> Result<()> {
    Ffmpeg::resolve(explicit).map(|_| ())
}

/// 在 PATH 中查找可执行的 `ffmpeg` / `ffmpeg.exe`。
fn which_ffmpeg() -> Result<PathBuf> {
    let candidates = ["ffmpeg", "ffmpeg.exe"];
    for name in candidates {
        if let Ok(output) = Command::new(name).arg("-version").output() {
            if output.status.success() {
                return Ok(PathBuf::from(name));
            }
        }
    }
    Err(AppError::FfmpegNotFound)
}
