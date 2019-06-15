use std::env;

use std::ptr;
use x11::xlib::{XCloseDisplay, XOpenDisplay, XScreenCount, XScreenOfDisplay};

use clap::ArgMatches;

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
}

impl Display {
    pub unsafe fn new() -> Result<Display, String> {
        // Open a display. This will fail if $DISPLAY isn't valid
        let display = XOpenDisplay(ptr::null());

        if display.is_null() {
            return Err("XOpenDisplay failed".to_string());
        };

        Ok(Display {
            display: display,
            screens: Vec::new(),
        })
    }
}

pub fn run(matches: ArgMatches) -> Result<String, String> {
    if let Err(e) = display() {
        return Err(e);
    }

    if let Err(e) = resolution() {
        return Err(e);
    }

    Ok("Wow".to_string())
}

/// Checks for the $DISPLAY env variable, which is required by X11
///
/// ```
/// use std::env;
///
/// env::set_var("DISPLAY", ":0");
/// assert_eq!(vodka::display().unwrap(), ":0");
/// ```
///
/// ```should_panic
/// use std::env;
///
/// env::set_var("DISPLAY", "");
/// assert_eq!(vodka::display().unwrap(), ":0");
/// ```
pub fn display() -> Result<String, String> {
    match env::var("DISPLAY") {
        Ok(value) => {
            println!("DISPLAY: {:?}", value);
            Ok(value)
        }
        Err(_) => Err(format!("$DISPLAY must be set! Aborting...")),
    }
}

pub fn resolution() -> Result<Vec<Screen>, String> {
    unsafe {
        // Open a display. This will fail if $DISPLAY isn't valid
        let display = XOpenDisplay(ptr::null());

        // Return the number of available screens
        let count_screens = XScreenCount(display);

        let mut screens = Vec::new();

        println!("Number of screens: {}", count_screens);

        for i in 0..count_screens {
            let screen = *XScreenOfDisplay(display, i);

            println!("\tScreen {}: {}x{}", i + 1, screen.width, screen.height);

            let screen = Screen {
                resolution: Resolution {
                    x: screen.width,
                    y: screen.height,
                },
            };
            screens.push(screen);
        }

        // close the display
        XCloseDisplay(display);

        return Ok(screens);
    }
}
