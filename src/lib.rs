use crate::backend::{Backend, BackendError};

#[cfg(not(any(feature = "sdl3", feature = "sdl2")))]
compile_error!("At least one backend feature should be enabled!");

#[cfg(all(feature = "sdl3", feature = "sdl2"))]
compile_error!("Can't activate both SDL2 and SDL3 backends at the same time!");

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
    #[cfg(feature = "sdl3")]
    backend: backend::SDL3Backend,
    /// SDL2 backend
    #[cfg(feature = "sdl2")]
    backend: backend::SDL2Backend,
}

impl LEDStripDisplay {
    /// Create and initialize a LEDStripDisplay window with the SDL3 backend.
    #[cfg(feature = "sdl3")]
    pub fn new(length: usize, led_per_row: u32, context: &sdl3::Sdl) -> Result<Self, BackendError> {
        Ok(LEDStripDisplay {
            length,
            backend: backend::SDL3Backend::new(600, 800, led_per_row, context)?,
        })
    }

    /// Create and initialize a LEDStripDisplay window with the SDL2 backend.
    #[cfg(feature = "sdl2")]
    pub fn new(length: usize, led_per_row: u32, context: &sdl2::Sdl) -> Result<Self, BackendError> {
        Ok(LEDStripDisplay {
            length,
            backend: backend::SDL2Backend::new(600, 800, led_per_row, context)?,
        })
    }

    pub fn update(&mut self, leds: &[LED]) -> Result<(), BackendError> {
        self.backend.update(leds)
    }
}
