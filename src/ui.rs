use std::{future::IntoFuture, sync::mpsc, thread};

use gtk4::{
    ffi::GtkFrame, gio::FILE_ATTRIBUTE_STANDARD_NAME, prelude::*, Application, ApplicationWindow, DrawingArea, FlowBox, Frame, Grid, ScrolledWindow
};
use crate::uiobjects::*;

pub fn build_ui() -> Result<Application, Box<dyn std::error::Error>> {
    let app = Application::builder()
        .application_id("org.zambieslar.ihyprselect")
        .build();

    app.connect_activate(|app| {
        let layout = Grid::builder()
            .column_spacing(4)
            .row_spacing(4)
            .build();
        let b_layout = Grid::builder()
            .build();
        let flow_box = FlowBox::builder()
            .max_children_per_line(4)
            .column_spacing(0)
            .row_spacing(0)
            .build();
        let main_window = ApplicationWindow::builder()
            .default_width(600)
            .default_height(400)
            .application(app)
            .resizable(false)
            .child(&b_layout)
            .child(&layout)
            .build();
        let set_button = button_builder("Apply");
        let src_button = button_builder("Source");
        let file_dialog = filed_builder();
        let swindow = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .has_frame(true)
            .build();
        swindow.set_child(Some(&flow_box));
        layout.attach(&src_button, 0, 0, 25, 7);
        layout.attach(&set_button, 0, 7, 25, 7);
        layout.attach(&b_layout, 25, 0, 100, 100);
        b_layout.attach(&swindow, 0, 0, 1, 1);
        main_window.present();
        src_button.connect_clicked(move |_| {
            let flow_box = flow_box.clone();
            let arc = std::sync::Arc::new(std::sync::Mutex::new(vec![]));
            let arcb = std::sync::Arc::clone(&arc);
            file_dialog.select_folder(Some(&main_window), gtk4::gio::Cancellable::NONE, move |file| {
                let data = arc.try_lock().unwrap();
                data.push(file);
                drop(data);
            });
            thread::spawn(move || {
                let mut data = arc.try_lock().unwrap();
                let entrys = data[0].unwrap().enumerate_children(FILE_ATTRIBUTE_STANDARD_NAME.as_str(), gtk4::gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS, gtk4::gio::Cancellable::NONE).unwrap();
                for entry in entrys {
                    match entry {
                        Ok(..) => {
                            let pixbuf = gdk4::gdk_pixbuf::Pixbuf::from_file_at_scale(format!(
                                "{}/{}",
                                data[0].unwrap().path().unwrap().to_str().unwrap(),
                                entry.expect("Unable to load file").name().to_str().expect("Failed to load file name")),
                                                                                      100,
                                                                                      75,
                                                                                      true).unwrap();
                            data.clear();
                            let texture = gdk4::Texture::for_pixbuf(&pixbuf);
                            data.push(texture);
                            drop(data);
                        }
                        Err(error) => {
                            println!("{:#?}", error)
                        }
                    }
                }
            });
            loop {
            if let Ok(mut data) = arcb.try_lock() {
                let picture = gtk4::Image::builder()
                    .vexpand(true)
                    .hexpand(true)
                    .build();
                flow_box.append(&picture);
                picture.set_from_paintable(Some(&data[0]));
                picture.connect_scale_factor_notify(move |picture|{
                    picture.queue_resize();
                });
            data.clear();
            drop(data);
            } else {
                continue
            };
            }
        });
    });
    Ok(app)
}
