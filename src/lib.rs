//#[macro_use]
//extern crate log;

//use std::env;
//use std::ptr;
//use std::thread;
//use std::time;

//use x11::xlib::{XCloseDisplay, XOpenDisplay, XScreenCount, XScreenOfDisplay, XSync};

//extern crate cairo;
//extern crate gdk;
//extern crate gio;
//extern crate gtk;

//use gio::prelude::*;
//use gtk::prelude::*;
//use gtk::{ApplicationWindow, Fixed, Button};


//struct Resolution {
//    x: i32,
//    y: i32,
//}

//pub struct Screen {
//    resolution: Resolution,
//}

//// X11 display wrapper, which manages all interactions with the X server.
//pub struct Display {
//    pub screens: Vec<Screen>,
//    pub display: *mut x11::xlib::Display,
//    pub display_env_var: String,
//}

//impl Display {
//    // Create a new display
//    pub unsafe fn new() -> Result<Display, String> {
//        trace!("Creating new display instance");

//        trace!("Checking for DISPLAY");
//        let display_env_var = match env::var("DISPLAY") {
//            Ok(value) => value,
//            Err(_) => return Err(format!("$DISPLAY must be set! Aborting...")),
//        };

//        trace!("Connecting to the X server");
//        let display = XOpenDisplay(ptr::null());

//        if display.is_null() {
//            return Err("XOpenDisplay failed".to_string());
//        };

//        let count_screens = XScreenCount(display);

//        info!("Number of screens: {}", count_screens);

//        let screens: Vec<Screen> = (0..count_screens)
//            .map(|i| {
//                let screen = *XScreenOfDisplay(display, i);

//                info!("\tScreen {}: {}x{}", i + 1, screen.width, screen.height);

//                Screen {
//                    resolution: Resolution {
//                        x: screen.width,
//                        y: screen.height,
//                    },
//                }
//            })
//            .collect();

//        trace!("Syncing with X server");
//        // This ensure we errors about XGrabKey and other failures
//        // before we try to daemonize
//        XSync(display, 0);

//        Ok(Display {
//            display,
//            screens,
//            display_env_var,
//        })
//    }

//    pub unsafe fn start(&mut self) {
//        trace!("Starting input loop");
//        loop {
//            trace!("Sleeping 5 seconds");
//            thread::sleep(time::Duration::from_millis(5_000));
//        }
//    }

//    pub unsafe fn doit(&mut self) {
//        ////autopilot::mouse::click(autopilot::mouse::Button::Right, None);
//        // unsafe {
//        //     let display = x11::xlib::XOpenDisplay(std::ptr::null());

//        //     let scale_factor = autopilot::screen::scale();
//        //     let screen = x11::xlib::XDefaultScreen(display);
//        //     let width = f64::from(x11::xlib::XDisplayWidth(display, screen));
//        //     let height = f64::from(x11::xlib::XDisplayHeight(display, screen));

//        //     info!("scale_factor: {}", scale_factor);
//        //     info!("screen: {}", screen);
//        //     info!("width: {}", width);
//        //     info!("height: {}", height);
//        // }

//        debug!("gonna be gud!");
//        let application = gtk::Application::new(
//            Some("com.github.gtk-rs.examples.transparent_main_window"),
//            Default::default(),
//        )
//        .expect("Initialization failed...");

//        application.connect_activate(|app| {
//            build_ui(app);
//        });
//        application.run(&args().collect::<Vec<_>>());

//        debug!("wow!");
//    }
//}

//impl Drop for Display {
//    fn drop(&mut self) {
//        unsafe {
//            trace!("Closing X display");
//            XCloseDisplay(self.display);
//        }
//    }
//}
//fn build_ui(application: &gtk::Application) {
//    let window = ApplicationWindow::new(application);
//    set_visual(&window, &None);

//    window.connect_draw(draw);

//    window.set_title("vodka");
//    window.set_default_size(500, 500);
//    window.set_app_paintable(true); // crucial for transparency

//    let fixed = Fixed::new();
//    window.add(&fixed);
//    let button = Button::new_with_label("Dummy");
//    button.set_size_request(100, 30);
//    fixed.add(&button);

//    window.show_all();
//}


//fn set_visual(window: &ApplicationWindow, _screen: &Option<gdk::Screen>) {
//    if let Some(screen) = window.get_screen() {
//        if let Some(visual) = screen.get_rgba_visual() {
//            window.set_visual(Some(&visual)); // crucial for transparency
//        }
//    }
//}

//fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
//    // crucial for transparency
//    ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4);
//    ctx.set_operator(cairo::Operator::Screen);
//    ctx.paint();
//    Inhibit(false)
//}



//pub unsafe fn run(matches: clap::ArgMatches) -> Result<String, String> {
//    // let mut display = match Display::new() {
//    //     Ok(display) => display,
//    //     Err(e) => {
//    //         return Err(e);
//    //     }
//    // };
//    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.transparent_main_window"),
//                                            Default::default())
//                                       .expect("Initialization failed...");

//    application.connect_activate(|app| {
//        build_ui(app);
//    });

//    application.run(&args().collect::<Vec<_>>());
//    println!("wow!");

//    // display.start();
//    // display.doit();

//    Ok("Sucess, much wow.".to_string())
//}



