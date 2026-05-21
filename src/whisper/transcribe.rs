use std::path::Path;

use whisper_rs::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
};

use crate::error::Result;
use crate::types::{centis_to_ms, Caption};
use crate::whisper::audio::load_wav;

/// Whisper 转写时的可配置项。
pub struct TranscribeOptions<'a> {
    /// GGML 模型文件路径（`.bin`）。
    pub model_path: &'a Path,
    /// 语言代码，如 `"zh"`、`"en"`；`None` 或 `"auto"` 表示自动检测。
    pub language: Option<&'a str>,
    /// 是否启用翻译模式（非英语 → 英语）。
    pub translate: bool,
}

/// 对 WAV 文件执行完整 Whisper 推理，返回带时间轴的字幕列表。
///
/// 内部流程：加载 WAV → 加载模型 → `state.full` → 遍历 [`whisper_rs::WhisperState::as_iter`]
/// 将每段文本映射为 [`Caption`]（空文本与无效时间窗会被跳过）。
///
/// # 线程数
///
/// 推理线程数取 `min(可用 CPU 数, 16)`，至少回退为 4。
pub fn transcribe(wav_path: &Path, options: &TranscribeOptions<'_>) -> Result<Vec<Caption>> {
    let audio = load_wav(wav_path)?;

    let ctx = WhisperContext::new_with_params(
        options.model_path.to_str().ok_or_else(|| {
            crate::error::AppError::InvalidPath(options.model_path.to_path_buf())
        })?,
        WhisperContextParameters::default(),
    )?;

    let mut state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

    params.set_n_threads(num_cpus());
    params.set_translate(options.translate);
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    match options.language {
        Some("auto") | None => params.set_language(None),
        Some(lang) => params.set_language(Some(lang)),
    }

    state.full(params, &audio)?;

    let mut captions = Vec::new();
    for segment in state.as_iter() {
        let text = segment.to_str_lossy()?.trim().to_string();
        if text.is_empty() {
            continue;
        }
        let start_ms = centis_to_ms(segment.start_timestamp());
        let end_ms = centis_to_ms(segment.end_timestamp());
        if end_ms <= start_ms {
            continue;
        }
        captions.push(Caption::new(start_ms, end_ms, text));
    }

    Ok(captions)
}

/// 供 Whisper 使用的推理线程数上限。
fn num_cpus() -> i32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as i32)
        .unwrap_or(4)
        .min(16)
}
