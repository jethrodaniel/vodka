#[macro_use]
extern crate log;

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Fixed};

pub struct App {
    application: gtk::Application,
}

impl App {
    /// Creates a new app instance
    ///
    /// ```
    /// let app = vodka::App::new();
    /// ```
    ///
    pub fn new() -> Result<App, String> {
        trace!("Creating new app instance");

        let application = gtk::Application::new(Some("com.jethrodaniel.vodka"), Default::default())
            .expect("Initialization failed...");

        application.connect_activate(|app| {
            App::build_ui(app);
        });

        Ok(App { application })
    }

    /// Starts the app
    ///
    /// ```no_run
    /// let mut app = match vodka::App::new() {
    ///   Ok(a) => a,
    ///   Err(e) => panic!("This should've worked"),
    /// };
    ///
    /// app.start();
    /// ```
    ///
    pub fn start(&mut self) -> Result<String, String> {
        trace!("Starting app");

        let args = Vec::new();
        self.application.run(&args);

        Ok("wow!".to_string())
    }

    fn build_ui(application: &gtk::Application) {
        trace!("Builing window and components");

        let window = ApplicationWindow::new(application);
        App::set_visual(&window, &None);

        window.connect_draw(App::draw);
        window.set_title("vodka");
        window.set_app_paintable(true); // crucial for transparency
        window.show_all();
    }

    fn set_visual(window: &ApplicationWindow, _screen: &Option<gdk::Screen>) {
        trace!("Setting main screen visibility");

        if let Some(screen) = window.get_screen() {
            if let Some(visual) = screen.get_rgba_visual() {
                window.set_visual(Some(&visual)); // crucial for transparency
            }
        }
    }

    fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
        trace!("Drawing");

        // crucial for transparency
        ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4); // 40%
        ctx.set_operator(cairo::Operator::Screen);
        ctx.paint();
        Inhibit(false)
    }
}

// Runs the parsed command line arguments, i.e, starts the program, basically
pub fn run(_matches: clap::ArgMatches) -> Result<String, String> {
    trace!("Running parsed args");

    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            return Err(e);
        }
    };

    if let Err(e) = app.start() {
        return Err(e);
    };

    Ok("Sucess, much wow.".to_string())
}

#[cfg(test)]
mod tests {

    #[test]
    fn internal_thing_works() {
      assert_eq!(true, true);
    }
}
