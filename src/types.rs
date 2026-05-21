//! 跨模块共享的领域类型与时间换算。

/// 一条带起止时间的字幕片段。
///
/// 时间轴统一使用**毫秒**，与 SRT 写入模块一致；
/// Whisper 原始输出为 centisecond（1 cs = 10 ms），经 [`centis_to_ms`] 转换后填入本结构。
#[derive(Debug, Clone)]
pub struct Caption {
    /// 字幕开始时间（毫秒，含）。
    pub start_ms: u64,
    /// 字幕结束时间（毫秒，不含或含取决于播放器；本库保证 `end_ms > start_ms`）。
    pub end_ms: u64,
    /// 字幕正文（已去除首尾空白）。
    pub text: String,
}

impl Caption {
    /// 构造一条字幕片段。
    ///
    /// # 参数
    ///
    /// - `start_ms` / `end_ms`：毫秒时间戳
    /// - `text`：任意可转为 [`String`] 的文本
    pub fn new(start_ms: u64, end_ms: u64, text: impl Into<String>) -> Self {
        Self {
            start_ms,
            end_ms,
            text: text.into(),
        }
    }
}

/// 将 Whisper 时间戳从 **centisecond**（10 ms 为单位）转为 **毫秒**。
///
/// Whisper API 中 `start_timestamp` / `end_timestamp` 返回值的单位为 centisecond；
/// 负数会被钳制为 0。
///
/// # 示例
///
/// ```
/// use video_subtitle::types::centis_to_ms;
/// assert_eq!(centis_to_ms(100), 1_000); // 100 cs = 1 s
/// ```
pub fn centis_to_ms(cs: i64) -> u64 {
    (cs.max(0) as u64) * 10
}
