use super::*;

use sdl3::{pixels::Color, render::FRect};

impl From<sdl3::Error> for BackendError {
    fn from(e: sdl3::Error) -> Self {
        Self {
            source: Box::new(e),
        }
    }
}

impl From<sdl3::video::WindowBuildError> for BackendError {
    fn from(e: sdl3::video::WindowBuildError) -> Self {
        Self {
            source: Box::new(e),
        }
    }
}

/// Display backend using SDL3.
///
/// LEDs are square displayed in grid. Each LED side size is
/// the `<window-width> / <led-per-row>`.
pub struct SDL3Backend {
    height: u32,
    width: u32,

    /// Size of the side of a led in pixel.
    led_size: f32,
    led_per_row: u32,

    canvas: sdl3::render::WindowCanvas,
}

impl SDL3Backend {
    /// Initialise a new SDL3 window for the led strip display.
    pub fn new(
        height: u32,
        width: u32,
        led_per_row: u32,
        context: &sdl3::Sdl,
    ) -> Result<Self, BackendError> {
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window("led-strip-display", width, height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        let gap = width % led_per_row;
        let base_side = width / led_per_row;

        Ok(SDL3Backend {
            height,
            width,
            led_size: (base_side as f32) + (gap as f32) / (led_per_row as f32),
            led_per_row,
            canvas,
        })
    }
}

impl Backend for SDL3Backend {
    /// Display the given LEDs range.
    fn update(&mut self, leds: &[LED]) -> Result<(), BackendError> {
        for (i, l) in leds.iter().enumerate() {
            let column = (i as u32 % self.led_per_row);
            let row = (i as u32 / self.led_per_row);

            self.canvas.set_draw_color(Color::RGB(l.r, l.g, l.b));
            self.canvas.fill_rect(FRect::new(
                column as f32 * self.led_size,
                row as f32 * self.led_size,
                self.led_size,
                self.led_size,
            ))?
        }

        self.canvas.present();
        Ok(())
    }
}
