use gtk4::{prelude::*, FileDialog, Application, ApplicationWindow, Box as GtkBox, Button, FixedLayout, Grid, GridLayout};

pub fn button_builder(label: &str) -> Button {
    let button = Button::builder()
        .width_request(-1)
        .height_request(-1)
        .label(label)
        .build();
    button
}

pub fn vbox_builder() -> GtkBox {
    let vbox = GtkBox::builder()
    .build();
    vbox
}

pub fn hbox_builder() -> GtkBox {
    let hbox = GtkBox::builder()

        .build();
    hbox
}

pub fn filed_builder() -> FileDialog {
    let file_dialog = FileDialog::builder()
        .title("Source Directory")
        .build();
    file_dialog
}
