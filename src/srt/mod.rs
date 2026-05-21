//! SubRip（`.srt`）字幕生成：时间格式化与文件写入。

mod format;
mod writer;

pub use format::format_timestamp;
pub use writer::write_srt;

use crate::types::Caption;

/// 将 [`Caption`] 切片序列化为 SubRip 文本（UTF-8，不含 BOM）。
///
/// 每条字幕块格式：
///
/// ```text
/// {序号}
/// {开始} --> {结束}
/// {正文}
///
/// ```
///
/// 时间戳分隔符为逗号（`,`），符合常见 SRT 约定。
pub fn captions_to_srt(captions: &[Caption]) -> String {
    let mut out = String::new();
    for (i, cap) in captions.iter().enumerate() {
        out.push_str(&(i + 1).to_string());
        out.push('\n');
        out.push_str(&format_timestamp(cap.start_ms));
        out.push_str(" --> ");
        out.push_str(&format_timestamp(cap.end_ms));
        out.push('\n');
        out.push_str(cap.text.trim());
        out.push_str("\n\n");
    }
    out
}
