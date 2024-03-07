use std::{path::Path, sync::{mpsc::channel, Arc, Mutex}};
use gtk4::{gio::ffi::{GCancellable, GFile}, gsk::ffi::GskPath, prelude::*, Application, ApplicationWindow, Button, FileChooserNative, Grid};
use crate::uiobjects::*;

pub fn load(path: &str) {
    let file = gtk4::gio::File::for_path(path);
}
pub fn build_ui() -> Result<Application, Box<dyn std::error::Error>> {
    let app = Application::builder()
        .application_id("org.zambieslar.ihyprselect")
        .build();

    app.connect_activate(|app| {
        let layout = Grid::builder()
            .build();
        let main_window = ApplicationWindow::builder()
            .default_width(600)
            .default_height(400)
            .resizable(false)
            .application(app)
            .child(&layout)
            .build();
        let vbox = vbox_builder();
        let set_button = button_builder("Apply");
        let src_button = button_builder("Source");
        let wp_frame = gtk4::Frame::builder()
            .build();
        let file_dialog = filed_builder();
        layout.attach(&src_button, 0, 0, 50, 50);
        layout.attach(&set_button, 0, 50, 50, 50);
        layout.attach(&wp_frame, 50, 0, 550, 50);
        let picture = gtk4::Image::new();
        wp_frame.set_child(Some(&picture));
        main_window.present();
        src_button.connect_clicked(move |_|{
            picture(Some(file_dialog.clone().open(Some(&main_window), gtk4::gio::Cancellable::NONE, |file|{

            })));
        });
    });
    Ok(app)
}
