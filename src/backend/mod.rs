use core::error::Error;
use core::fmt;

mod sdl3_backend;

// Re-exports
pub use sdl3_backend::SDL3Backend;

pub trait Backend {
    // fn update();
}

/// Wrapper for errors returned by the backend.
#[derive(Debug)]
pub struct BackendError {
    source: Box<dyn Error + 'static>,
}

impl Error for BackendError {
    /// Return error source.
    /// BackendError is always caused by another error, so this method cannot return None.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backend error: {}", self.source().unwrap())
    }
}
