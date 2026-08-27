//! Stand-in for [`super::whisper::WhisperTranscriber`] when the
//! `whisper-backend` feature is off.
//!
//! whisper-rs-sys regenerates its bindings at build time and current bindgen
//! miscompiles them against newer libclang (observed on Arch, clang 21:
//! `_IO_FILE` and `whisper_full_params` layout asserts fail). This build's
//! transcripts come from an external ASR pipeline instead, so the in-app
//! backend degrades to a runtime error rather than blocking compilation.
//! Everything else — model download UI, caption plumbing, timeline
//! transcript maths — compiles and behaves unchanged.

use std::path::Path;

use super::{TranscribeOptions, Transcriber, TranscriptionResult};
use crate::decode::pcm::PcmBuffer;
use crate::error::{MediaError, Result};

const DISABLED: &str =
    "this build was compiled without the whisper backend (feature `whisper-backend`)";

pub struct WhisperTranscriber {
    _private: (),
}

impl WhisperTranscriber {
    pub fn from_model_path(_path: &Path) -> Result<Self> {
        Err(MediaError::Transcribe(DISABLED.to_string()))
    }

    pub fn with_threads(self, _threads: i32) -> Self {
        self
    }
}

impl Transcriber for WhisperTranscriber {
    fn transcribe_pcm(
        &self,
        _pcm: &PcmBuffer,
        _opts: &TranscribeOptions,
    ) -> Result<TranscriptionResult> {
        Err(MediaError::Transcribe(DISABLED.to_string()))
    }
}
