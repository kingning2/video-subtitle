use std::path::Path;

use hound::{SampleFormat, WavReader};
use whisper_rs::convert_integer_to_float_audio;

use crate::error::{AppError, Result};

/// 从磁盘读取 WAV 并转为 Whisper 推理所需的 `f32` 样本向量。
///
/// # 格式要求
///
/// | 属性     | 要求        |
/// |----------|-------------|
/// | 采样率   | 16 000 Hz   |
/// | 声道     | 1（单声道） |
/// | 样本格式 | 16-bit 整数或 32-bit 浮点 |
///
/// 整数 PCM 会通过 `whisper_rs::convert_integer_to_float_audio` 归一化到 `[-1.0, 1.0]`。
///
/// # 错误
///
/// 采样率/声道不符、文件损坏或转换失败时返回 [`AppError::Audio`] 或 [`AppError::Whisper`]。
pub fn load_wav(path: &Path) -> Result<Vec<f32>> {
    let reader = WavReader::open(path).map_err(|e| AppError::Audio(e.to_string()))?;
    let spec = reader.spec();

    if spec.sample_rate != 16_000 {
        return Err(AppError::Audio(format!(
            "需要 16kHz 采样率，当前为 {}Hz",
            spec.sample_rate
        )));
    }

    if spec.channels != 1 {
        return Err(AppError::Audio(format!(
            "需要单声道，当前为 {} 声道",
            spec.channels
        )));
    }

    match spec.sample_format {
        SampleFormat::Int => {
            let samples: Vec<i16> = reader
                .into_samples::<i16>()
                .map(|s| s.map_err(|e| AppError::Audio(e.to_string())))
                .collect::<Result<_>>()?;
            let mut audio = vec![0.0f32; samples.len()];
            convert_integer_to_float_audio(&samples, &mut audio)?;
            Ok(audio)
        }
        SampleFormat::Float => {
            let samples: Vec<f32> = reader
                .into_samples::<f32>()
                .map(|s| s.map_err(|e| AppError::Audio(e.to_string())))
                .collect::<Result<_>>()?;
            Ok(samples)
        }
    }
}
