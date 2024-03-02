use gtk4::{Application, ApplicationWindow, prelude::*};
use std::{self, os::unix::net::*};
mod ui;

fn main() {
    let app = Application::builder()
        .application_id("org.zambieslar.ihyprselect")
        .build();
    app.connect_activate(ui::build_ui);
    app.run();

}
