use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::error::Result;
use crate::srt::captions_to_srt;
use crate::types::Caption;

/// 将字幕列表写入 SubRip 文件。
///
/// 若 `path` 已存在会被覆盖（[`std::fs::File::create`]）。
/// 内容编码为 UTF-8 字节，无 BOM。
pub fn write_srt(path: &Path, captions: &[Caption]) -> Result<()> {
    let content = captions_to_srt(captions);
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
