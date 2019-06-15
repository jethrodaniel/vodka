use std::env;
use std::ptr;

use x11::xlib::{
  XCloseDisplay,
  XOpenDisplay,
  XScreenCount,
  XScreenOfDisplay,
  XSync,
};

struct Resolution {
    x: i32,
    y: i32,
}

pub struct Screen {
    resolution: Resolution,
}

// X11 display wrapper, which manages all interactions with the X server.
pub struct Display {
    pub screens: Vec<Screen>,
    pub display: *mut x11::xlib::Display,
    pub display_env_var: String,
}

impl Display {
    // Create a new display
    pub unsafe fn new() -> Result<Display, String> {
        let display_env_var = match env::var("DISPLAY") {
            Ok(value) => value,
            Err(_) => return Err(format!("$DISPLAY must be set! Aborting...")),
        };

        let display = XOpenDisplay(ptr::null());

        if display.is_null() {
            return Err("XOpenDisplay failed".to_string());
        };

        let count_screens = XScreenCount(display);

        println!("Number of screens: {}", count_screens);

        let screens: Vec<Screen> = (0..count_screens)
            .map(|i| {
                let screen = *XScreenOfDisplay(display, i);

                println!("\tScreen {}: {}x{}", i + 1, screen.width, screen.height);

                Screen {
                    resolution: Resolution {
                        x: screen.width,
                        y: screen.height,
                    },
                }
            })
            .collect();


        // Sync with the X server.
        // This ensure we errors about XGrabKey and other failures
        // before we try to daemonize
        XSync(display, 0);

        Ok(Display {
            display,
            screens,
            display_env_var,
        })
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.display);
        }
    }
}

pub unsafe fn run(matches: clap::ArgMatches) -> Result<String, String> {
    if let Err(e) = Display::new() {
        return Err(e);
    }

    Ok("Sucess, much wow.".to_string())
}
