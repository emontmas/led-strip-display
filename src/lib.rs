use crate::backend::{Backend, BackendError};

/// Module containing low-level implementation for LED strip display.
mod backend;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct LED {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type LEDStrip = Vec<LED>;

pub struct LEDStripDisplay {
    /// Number of LED in the strip
    length: usize,
    /// SDL3 backend
    backend: backend::SDL3Backend,
}

impl LEDStripDisplay {
    /// Create and initialize a LEDStripDisplay window with the SDL3 backend.
    pub fn new(length: usize, context: &sdl3::Sdl) -> Result<Self, BackendError> {
        Ok(LEDStripDisplay {
            length,
            backend: backend::SDL3Backend::new(600, 800, context)?,
        })
    }

    pub fn update(&mut self, leds: &[LED]) -> Result<(), BackendError> {
        self.backend.update(leds)
    }
}
