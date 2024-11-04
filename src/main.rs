use crate::definitions::*;
use crate::traits::*;
use gtk4::gio::{Cancellable, FileQueryInfoFlags, FILE_ATTRIBUTE_STANDARD_NAME};
use gtk4::glib::ControlFlow;
use gtk4::{prelude::*, Image};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use ui::MainWindow;
mod definitions;
mod traits;
mod ui;
mod uiobjects;

fn main() {
    let app = gtk4::Application::builder()
        .application_id("org.zambieslar.ihyprselect")
        .build();
    app.connect_activate(move |app| {
        let environment: Arc<Mutex<Environment>> = Arc::new(Mutex::new(Environment::init()));

        // Initialize main window and children //
        let main_window = MainWindow::build_ui(app.clone());
        let main_windowb = main_window.clone();
        main_window
            .main_window
            .set_child(Some(&main_window.a_layout));
        main_window.set_button.set_halign(gtk4::Align::Center);
        main_window.src_button.set_halign(gtk4::Align::Center);
        main_window.src_button.default_size(100, 40);
        main_window.set_button.default_size(100, 40);
        main_window.swindow.set_child(Some(&main_window.flow_box));
        main_window.vbox.append(&main_window.src_button);
        main_window.vbox.append(&main_window.set_button);
        main_window.a_layout.attach(&main_window.vbox, 0, 0, 1, 1);
        main_window
            .a_layout
            .attach(&main_window.b_layout, 1, 0, 1, 1);
        main_window
            .b_layout
            .attach(&main_window.swindow, 0, 0, 1, 1);
        // Initialize main window and children //

        // Start of events //
        /* Selecting image sets the static variable to an index in the flowboxes array. Index is
        used to index the IMAGES array contianing the images loaded into memory */
        main_window
            .flow_box
            .connect_child_activated(move |_parent, child| unsafe {
                SELECTED = child.index();
                thread::spawn(move || {
                    if let Ok(mut preload) = std::process::Command::new("/usr/bin/hyprctl")
                        .arg(format!("hyprpaper preload {}", IMAGES[SELECTED as usize]))
                        .spawn()
                    {
                        preload.wait().unwrap();
                        if let Ok(mut preset) = std::process::Command::new("/usr/bin/hyprctl")
                            .arg(format!(
                                "hyprpaper wallpaper ,{}",
                                IMAGES[SELECTED as usize]
                            ))
                            .spawn()
                        {
                            preset.wait().unwrap();
                        };
                    }
                });
            });

        main_window.src_button.connect_clicked(move |_| {
            let flowbox = main_window.flow_box.clone();
            let file_dialog = main_window.file_dialog.clone();
            let (tx, rx) = mpsc::channel::<Option<String>>();
            file_dialog.select_folder(
                Some(&main_windowb.main_window),
                Cancellable::NONE,
                move |file| {
                    if let Ok(file) = file {
                        std::thread::spawn(move || {
                            let children = file
                                .enumerate_children(
                                    FILE_ATTRIBUTE_STANDARD_NAME,
                                    FileQueryInfoFlags::NONE,
                                    Cancellable::NONE,
                                )
                                .unwrap();
                            for child in children {
                                let loc = format!(
                                    "{}/{}",
                                    file.path().unwrap().to_str().unwrap(),
                                    child.unwrap().name().to_str().unwrap()
                                );
                                tx.send(Some(loc)).unwrap();
                            }
                            tx.send(None).unwrap();
                        });
                    }
                },
            );
            gtk4::glib::idle_add_local(move || match rx.try_recv() {
                Ok(Some(data)) => {
                    let image = Image::from_file(data.clone());
                    image.set_pixel_size(200);
                    unsafe {
                        IMAGES.push(data);
                    }
                    flowbox.append(&image);
                    ControlFlow::Continue
                }
                Ok(None) => ControlFlow::Break,
                Err(_err) => ControlFlow::Continue,
            });
        });
        main_window.set_button.connect_clicked(move |_| unsafe {
            let data = environment.lock().unwrap();
            update_config(data, IMAGES[SELECTED as usize].clone());
        });

        main_window.main_window.present();
    });
    app.run();
}
