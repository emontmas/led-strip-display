use super::*;

use sdl3::pixels::Color;

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

pub struct SDL3Backend {
    height: u32,
    width: u32,
    canvas: sdl3::render::WindowCanvas,
}

impl SDL3Backend {
    pub fn new(height: u32, width: u32, context: &sdl3::Sdl) -> Result<Self, BackendError> {
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window("led-strip-display", width, height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(SDL3Backend {
            height,
            width,
            canvas,
        })
    }
}
