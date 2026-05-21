/// 将毫秒时间戳格式化为 SRT 时间轴字符串 `HH:MM:SS,mmm`。
///
/// # 示例
///
/// ```
/// use video_subtitle::srt::format_timestamp;
/// assert_eq!(format_timestamp(1_234), "00:00:01,234");
/// ```
pub fn format_timestamp(ms: u64) -> String {
    let hours = ms / 3_600_000;
    let minutes = (ms % 3_600_000) / 60_000;
    let seconds = (ms % 60_000) / 1_000;
    let millis = ms % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02},{millis:03}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_zero() {
        assert_eq!(format_timestamp(0), "00:00:00,000");
    }

    #[test]
    fn formats_with_millis() {
        assert_eq!(format_timestamp(1_234), "00:00:01,234");
    }

    #[test]
    fn formats_long_duration() {
        assert_eq!(format_timestamp(3_661_500), "01:01:01,500");
    }
}
