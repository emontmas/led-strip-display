use crate::*;
use std::sync::atomic::AtomicBool;

/// True if an Sdl context was already created and forgot to prevent
/// the Rust crate from calling SDL_Quit() when the last Sdl context is dropped.
/// This let the C program responsible for managing SDL.
static HOLD_SDL: AtomicBool = AtomicBool::new(false);

/// When using the C API, the crate should not be responsible for initialization
/// and uninitialization of the SDL context (SDL_Init() / SDL_Quit()).
/// With the sdl crate, this is done through the Sdl struct initialization.
/// However this struct is still necessary to use the crate.
/// Also, there is no way to ensure usage of the SDL API (C or Rust) from
/// a single "main" thread in C.
/// Therefore the first call to this method creates and forget an Sdl context
/// to force internal crate ref counter to always be >1.
#[cfg(feature = "sdl3")]
fn get_sdl3_context() -> Result<sdl3::VideoSubsystem, sdl3::Error> {
    if !HOLD_SDL.fetch_or(true, std::sync::atomic::Ordering::Relaxed) {
        let c = sdl3::init().unwrap();
        std::mem::forget(c); // Increment SDL_COUNT, but never call drop() to decrease it.
    }

    sdl3::init()?.video()
}

#[no_mangle]
#[cfg(feature = "sdl3")]
pub extern "C" fn led_strip_display_new(
    length: cty::size_t,
    led_per_row: cty::uint32_t,
) -> *mut LEDStripDisplay {
    let video_subsys = match get_sdl3_context() {
        Ok(s) => s,
        Err(e) => {
            println!("Can't create LED strip display with SDL3: {e}");
            return std::ptr::null_mut();
        }
    };

    let display = match LEDStripDisplay::new(length, led_per_row, &video_subsys) {
        Ok(d) => d,
        Err(e) => {
            println!("Can't create LED strip display with SDL3: {e}");
            return std::ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(display))
}
