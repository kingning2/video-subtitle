---
name: video-subtitle
description: >-
  Develop, run, debug, and publish the video-subtitle Rust crate (FFmpeg audio
  extract → Whisper GGML ASR → SRT → FFmpeg burn-in). Use when working in this
  repo, video-subtitle CLI, video_subtitle library, whisper-rs, subtitle pipeline,
  crates.io publish for kingning2/video-subtitle, or Chinese/English auto subtitles.
---

# video-subtitle

Rust crate + CLI: **FFmpeg → Whisper → SRT → FFmpeg hard subs**.

- Repo: https://github.com/kingning2/video-subtitle
- Crate: `video-subtitle` / lib `video_subtitle`

## Architecture (do not break without reason)

| Step | Module | Key API |
|------|--------|---------|
| 1 | `ffmpeg::extract` | `extract_audio` — 16 kHz mono `pcm_s16le` WAV |
| 2 | `whisper::transcribe` | `transcribe` — GGML model, outputs `Caption` |
| 3 | `srt` | `write_srt` / `captions_to_srt` |
| 4 | `ffmpeg::burn` | `burn_subtitles` — `subtitles=` filter |
| Orchestration | `pipeline::run` | wires `Cli` → four steps |

**Time units**: Whisper segments use centiseconds; `types::centis_to_ms` → milliseconds for SRT.

## Code conventions

- Split by mod: `cli`, `error`, `pipeline`, `ffmpeg/{extract,burn,util}`, `whisper/{audio,transcribe}`, `srt/{format,writer}`, `types`.
- Errors: `AppError` + `Result<T>` in `error.rs`; map `whisper_rs::WhisperError`, FFmpeg stderr in `FfmpegFailed`.
- Keep changes minimal; match existing doc comments (`///` / `//!`) and Chinese CLI help where present.
- `whisper-rs` loads at runtime; audio must be **16 kHz mono** (enforced in `whisper/audio.rs`).

## Windows: subtitle burn (critical)

FFmpeg `subtitles=` misparses `C:/Users/...` (colon → wrong option, error `original_size` + `/Users/...`).

**Correct approach** (see `ffmpeg/burn.rs`):

1. `ffmpeg.run_in(Some(srt.parent()), args)` — cwd = SRT directory
2. Filter: `subtitles={filename}` — **basename only**, e.g. `20260521-191608.tmp.srt`
3. Input video path: **canonical absolute**; output path: absolute

Do **not** pass full `C:/...` paths inside the filter string on Windows.

## Running locally

```bash
cargo build --release
./target/release/video-subtitle -i video.mp4 -m /path/to/ggml-small.bin -l zh
```

| Flag | Notes |
|------|-------|
| `-l zh` | Chinese; prefer over `auto` for 中文 |
| `--translate` | Only for EN output; **never** for 中文字幕 |
| `--srt-only` | Skip burn; keeps flow for debugging ASR |
| `--keep-temp` | Keep `{stem}.tmp.wav` / `.tmp.srt` |
| `--ffmpeg` | If not on PATH |

**Models**: GGML `.bin` from [whisper.cpp models](https://huggingface.co/ggerganov/whisper.cpp). `ggml-base` works but 中文 quality is weak; recommend **`ggml-small`** or larger. Not `.pt` files.

**Build deps**: FFmpeg (runtime), CMake + C++ (compile `whisper-rs`). After code changes, user must `cargo build --release` — debug binary may be stale.

## Common failures

| Symptom | Cause | Fix |
|---------|-------|-----|
| `original_size` / `/Users/...` on burn | Windows filter path | Relative SRT name + `run_in(cwd)` |
| `FfmpegNotFound` | FFmpeg missing | Install FFmpeg or `--ffmpeg` |
| `需要 16kHz` | Wrong WAV | Re-run extract step |
| 识别文字不对 | `base` model / wrong lang | `ggml-small.bin`, `-l zh`, no `--translate` |
| `cargo publish` dirty tree | Uncommitted files | `git add` + `git commit` |
| publish 403 | Token scope | New token with **Publish new crates** |
| publish 400 email | crates.io | Verify email at settings/profile |
| publish 400 keywords | crates.io limit | **Max 5** `keywords` in `Cargo.toml` |

## Extending the project

- New CLI flags → `cli.rs` + wire in `pipeline.rs` / `TranscribeOptions`.
- Whisper tuning → `whisper/transcribe.rs` (`FullParams`, sampling strategy).
- Soft subs / rotation → new ffmpeg path; video may have `rotation -90` metadata (not handled today).
- Library users: `video_subtitle::pipeline::run(&Cli::parse())`.

## Publishing (crates.io)

1. `authors` / `repository` in `Cargo.toml` → `kingning2`, `https://github.com/kingning2/video-subtitle`
2. `git commit` all changes before `cargo publish` (no `--allow-dirty` for release)
3. `cargo login` with publish-new token; verified email on crates.io
4. `cargo publish` — version is immutable; bump `version` for fixes

See [reference.md](reference.md) for file map and publish checklist.

## Verification after edits

```bash
cargo test
cargo build --release
cargo package   # requires clean git
```

Manual smoke (needs FFmpeg + model):

```bash
video-subtitle -i sample.mp4 -m ggml-small.bin -l zh --keep-temp
```
