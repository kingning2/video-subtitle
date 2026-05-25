# video-subtitle reference

## Source layout

```
src/
├── main.rs           # CLI entry, path checks, check_ffmpeg
├── lib.rs            # re-exports
├── cli.rs            # clap Cli
├── error.rs          # AppError
├── pipeline.rs       # run(), PipelineOutput
├── types.rs          # Caption, centis_to_ms
├── ffmpeg/
│   ├── util.rs       # Ffmpeg::resolve, run, run_in
│   ├── extract.rs    # -vn -ar 16000 -ac 1 pcm_s16le
│   └── burn.rs       # subtitles filter, Windows cwd trick
├── whisper/
│   ├── audio.rs      # load_wav → Vec<f32>
│   └── transcribe.rs # WhisperContext, Greedy best_of 1
└── srt/
    ├── format.rs     # HH:MM:SS,mmm
    ├── mod.rs        # captions_to_srt
    └── writer.rs     # write file
```

## Temporary files

| File | Location |
|------|----------|
| `{stem}.tmp.wav` | Same dir as input video |
| `{stem}.tmp.srt` | Same dir |
| `{stem}_subtitled.mp4` | Default output (unless `-o`) |

Cleanup: without `--keep-temp`, WAV always deleted after run; SRT deleted after successful burn.

## FFmpeg commands (equivalent)

Extract:

```text
ffmpeg -y -i INPUT -vn -acodec pcm_s16le -ar 16000 -ac 1 OUTPUT.wav
```

Burn (from SRT directory):

```text
ffmpeg -y -i ABS_INPUT -vf subtitles=FILE.srt -c:a copy ABS_OUTPUT
```

## crates.io metadata constraints

- `keywords`: max **5**
- `categories`: typically 1–2
- `license`: SPDX e.g. `MIT`
- Publish requires verified email + token with `publish-new`

## Whisper segment → SRT block

```
index (1-based)
HH:MM:SS,mmm --> HH:MM:SS,mmm
text

```

Empty segments and `end_ms <= start_ms` are skipped in `transcribe.rs`.
