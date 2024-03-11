use gtk4::{
    gio::FILE_ATTRIBUTE_STANDARD_NAME, prelude::*, Application, ApplicationWindow, FlowBox, Frame, Grid, ScrolledWindow
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
        src_button.connect_clicked(move |_|{
            let flow_box = flow_box.clone();
            file_dialog.select_folder(Some(&main_window), gtk4::gio::Cancellable::NONE, move |file| {
                let entrys = file.clone().unwrap().enumerate_children(FILE_ATTRIBUTE_STANDARD_NAME.as_str(), gtk4::gio::FileQueryInfoFlags::NOFOLLOW_SYMLINKS, gtk4::gio::Cancellable::NONE).unwrap();
                for entry in entrys {
                    match entry {
                        Ok(..) => {
                            let texture = gdk4::Texture::from_filename(format!(
                                "{}/{}",
                                file.clone().unwrap().path().unwrap().to_str().unwrap(),
                                entry.expect("Unable to load file").name().to_str().expect("Failed to load file name")
                            )).unwrap();
                            let picture = gtk4::Image::builder()

                                .width_request(100)
                                .height_request(65)
                                .build();
                            picture.set_from_paintable(Some(&texture));
                            flow_box.append(&picture);
                        }
                        Err(error) => {
                            println!("{:#?}", error)
                        }
                    }
                }
            });
        });
    });
    Ok(app)
}
