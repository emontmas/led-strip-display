use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;

fn main() {
    let sdl_context = sdl3::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let display = led_strip_test::LEDStripDisplay::new(200, &sdl_context);

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

        ::std::thread::sleep(Duration::from_secs(1) / 60);
    }
}
