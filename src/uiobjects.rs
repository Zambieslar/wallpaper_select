use gtk4::{prelude::*, Box as GtkBox, Button, FileDialog};

pub fn button_builder(label: &str) -> Button {
    let button = Button::builder().label(label).build();
    button
}

pub fn vbox_builder() -> GtkBox {
    let vbox = GtkBox::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();
    vbox
}

pub fn hbox_builder() -> GtkBox {
    let hbox = GtkBox::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .build();
    hbox
}

pub fn filed_builder() -> FileDialog {
    let file_dialog = FileDialog::builder().title("Source Directory").build();
    file_dialog
}
