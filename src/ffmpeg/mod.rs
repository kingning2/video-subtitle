//! FFmpeg 子进程封装：音频提取与字幕烧录。
//!
//! 本模块不链接 FFmpeg 库，而是通过 [`Ffmpeg::run`] 调用系统已安装的 `ffmpeg` 可执行文件。

mod burn;
mod extract;
mod util;

pub use burn::burn_subtitles;
pub use extract::extract_audio;
pub use util::{check_ffmpeg, Ffmpeg};
