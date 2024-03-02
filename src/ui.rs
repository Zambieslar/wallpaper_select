use gtk4::{Application, ApplicationWindow, Grid, Button, GtkBox, prelude::*};

fn button_builder(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .build();
    button
}

fn vbox_builder() -> GtkBox {
    let vbox = GtkBox::new(gtk4::Orientation::Vertical, 0);
    GtkBox
}

pub fn build_ui(app: &Application) {
    let main_window = ApplicationWindow::builder()
        .default_width(600)
        .default_height(400)
        .resizable(true)
        .build();
    let main_grid = Grid::new();



    main_window.show();
}
