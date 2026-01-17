use crate::LED;
use core::error::Error;
use core::fmt;

#[cfg(feature = "sdl3")]
mod sdl3_backend;

// Re-exports
#[cfg(feature = "sdl3")]
pub use sdl3_backend::SDL3Backend;

pub trait Backend {
    fn update(&mut self, leds: &[LED]) -> Result<(), BackendError>;
}

/// Wrapper for errors returned by the backend.
#[derive(Debug)]
pub enum BackendError {
    ErrorSource(Box<dyn Error + 'static>),
    Str(String),
}

impl Error for BackendError {
    /// Return error source, or None if the BackendError contains a String.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BackendError::ErrorSource(s) => Some(s.as_ref()),
            BackendError::Str(_) => None,
        }
    }
}

impl fmt::Display for BackendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BackendError::ErrorSource(s) => write!(f, "Backend error: {}", s),
            BackendError::Str(s) => write!(f, "Backend error: {}", s),
        }
    }
}

impl From<String> for BackendError {
    fn from(s: String) -> Self {
        BackendError::Str(s)
    }
}
