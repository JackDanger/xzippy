use std::fmt;
use std::io;
use thiserror::Error;

/// All errors produced by xzippy.
#[derive(Error, Debug)]
pub enum XzippyError {
    /// Returned by every stub until the real implementation lands.
    #[error("not yet implemented")]
    NotYetImplemented,

    /// Wraps an underlying IO error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Error from the lzma-rust2 backend (Phase 1).
    #[error("LZMA2 backend error: {0}")]
    Backend(String),

    /// The LZMA2 stream properties byte was out of range.
    #[error("invalid properties byte: {0:#04x}")]
    InvalidProperties(u8),

    /// The input was truncated before the stream end marker.
    #[error("truncated LZMA2 stream")]
    Truncated,
}

impl XzippyError {
    /// Construct a [`Backend`](XzippyError::Backend) error from any `Display` value.
    pub fn backend<T: fmt::Display>(msg: T) -> Self {
        XzippyError::Backend(msg.to_string())
    }
}

/// Convenience alias used throughout xzippy.
pub type XzippyResult<T> = Result<T, XzippyError>;
