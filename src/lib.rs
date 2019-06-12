use std::ptr;
use x11::xlib::{XCloseDisplay, XOpenDisplay, XScreenCount, XScreenOfDisplay};

pub fn resolution() {
    unsafe {
        // open a display
        let display = XOpenDisplay(ptr::null());
        // return the number of available screens
        let count_screens = XScreenCount(display);

        println!("Number of screens: {}", count_screens);
        for i in 0..count_screens {
            let screen = *XScreenOfDisplay(display, i);
            println!("\tScreen {}: {}x{}", i + 1, screen.width, screen.height);
        }
        // close the display
        XCloseDisplay(display);
    }
}


