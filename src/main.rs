// #[macro_use]
// extern crate clap;
// use clap::{App, AppSettings, Arg, SubCommand};

// #[macro_use]
// extern crate log;
// extern crate simplelog;
// use simplelog::*;
// use std::fs::File;

// // Setup simplelog logging
// fn setup_logging(verbose: bool) {
//     let log_level = if verbose {
//         LevelFilter::Trace
//     } else {
//         LevelFilter::Debug
//     };

//     CombinedLogger::init(vec![
//         TermLogger::new(log_level, Config::default(), TerminalMode::Mixed).unwrap(),
//         WriteLogger::new(
//             log_level,
//             Config::default(),
//             File::create(format!("{}.log", crate_name!())).unwrap(),
//         ),
//     ])
//     .unwrap();
// }

// fn main() {
//     let matches = App::new(crate_name!())
//         .setting(AppSettings::SubcommandRequiredElseHelp)
//         .author(crate_authors!("\n"))
//         .version(crate_version!())
//         .about(crate_description!())
//         .subcommand(
//             SubCommand::with_name("start")
//                 .author(crate_authors!("\n"))
//                 .about("Starts the program")
//                 .version(crate_version!())
//                 .arg(
//                     Arg::with_name("verbose")
//                         .long("verbose")
//                         .short("v")
//                         .multiple(true),
//                 ),
//         )
//         .get_matches();

//     setup_logging(true);
//     std::env::set_var("G_MESSAGES_DEBUG", "all");
//     // std::env::set_var("G_DBUS_DEBUG", "all");

//     match matches.subcommand() {
//         ("start", Some(clone_matches)) => match clone_matches.occurrences_of("verbose") {
//             0 => (),
//             1 => {
//                 std::env::set_var("RUST_LOG", "info");
//             }
//             _ => {
//                 std::env::set_var("RUST_LOG", "trace");
//             }
//         },
//         (_, _) => (),
//     }

//     unsafe {
//         match vodka::run(matches) {
//             Ok(_) => (),
//             Err(e) => {
//                 error!("Application error: {}", e);
//                 std::process::exit(1)
//             }
//         }
//     }
// }
//
//

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Fixed, Button};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    set_visual(&window, &None);

    // window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    window.set_title("vodka");
    window.set_default_size(500, 500);
    window.set_app_paintable(true); // crucial for transparency

    let fixed = Fixed::new();
    window.add(&fixed);
    let button = Button::new_with_label("Dummy");
    button.set_size_request(100, 30);
    fixed.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.gtk-rs.examples.transparent_main_window"),
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    let args = Vec::new();
    application.run(&args);
    println!("wow!");
}

fn set_visual(window: &ApplicationWindow, _screen: &Option<gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(visual) = screen.get_rgba_visual() {
            window.set_visual(Some(&visual)); // crucial for transparency
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // crucial for transparency
    ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}
