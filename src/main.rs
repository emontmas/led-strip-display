use core::f32;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

use f32::consts::PI;

use led_strip_test::*;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let strip_length = 40;
    let mut strip: LEDStrip = vec![LED::default(); strip_length];

    let mut display = led_strip_test::LEDStripDisplay::new(strip_length, &sdl_context).unwrap();

    let mut t = 0;
    let t_max = 60;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let t_ratio = (t as f32) / (t_max as f32);

        // Update leds
        for (i, l) in strip.iter_mut().enumerate() {
            let led_phase = 2.0 * PI * (i as f32 / strip_length as f32);

            *l = LED {
                r: (255.0 * (f32::sin(2.0 * PI * t_ratio + led_phase) + 1.0) / 2.0) as u8,
                g: (255.0 * (f32::sin(2.0 * PI * t_ratio + (2.0 / 3.0) * PI + led_phase) + 1.0)
                    / 2.0) as u8,
                b: (255.0 * (f32::sin(2.0 * PI * t_ratio + (4.0 / 3.0) * PI + led_phase) + 1.0)
                    / 2.0) as u8,
            };
        }
        display.update(&mut strip).unwrap();

        t = (t + 1) % t_max;

        ::std::thread::sleep(Duration::from_secs(1) / 60);
    }
}
