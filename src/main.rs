use gtk4::prelude::*;
mod ui;
mod uiobjects;

fn main() {
    let app = ui::build_ui();
    app.unwrap().run();
}
