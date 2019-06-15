use std::env;
use std::ptr;

use x11::xlib::{XCloseDisplay, XOpenDisplay, XScreenCount, XScreenOfDisplay};

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
    pub unsafe fn new() -> Result<Display, String> {
        let display_env_var = match env::var("DISPLAY") {
            Ok(value) => value,
            Err(_) => return Err(format!("$DISPLAY must be set! Aborting...")),
        };

        let display = XOpenDisplay(ptr::null());

        if display.is_null() {
            return Err("XOpenDisplay failed".to_string());
        };

        // Return the number of available screens
        let count_screens = XScreenCount(display);

        let mut screens = Vec::new();

        println!("Number of screens: {}", count_screens);

        for i in 0..count_screens {
            let screen = *XScreenOfDisplay(display, i);

            println!("\tScreen {}: {}x{}", i + 1, screen.width, screen.height);

            screens.push(Screen {
                resolution: Resolution {
                    x: screen.width,
                    y: screen.height,
                },
            });
        }

        Ok(Display {
            display: display,
            screens: Vec::new(),
            display_env_var: display_env_var,
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
