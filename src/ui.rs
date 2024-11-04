use crate::uiobjects::*;
use gtk4::Box;
use gtk4::Button;
use gtk4::FileDialog;
use gtk4::{Application, ApplicationWindow, FlowBox, Grid, ScrolledWindow};

#[derive(Clone)]
pub struct MainWindow {
    pub a_layout: Grid,
    pub b_layout: Grid,
    pub flow_box: FlowBox,
    pub set_button: Button,
    pub src_button: Button,
    pub file_dialog: FileDialog,
    pub swindow: ScrolledWindow,
    pub vbox: Box,
    pub main_window: ApplicationWindow,
}

impl MainWindow {
    pub fn build_ui(app: Application) -> Self {
        let main_window = MainWindow {
            a_layout: Grid::builder().row_spacing(4).build(),
            b_layout: Grid::builder().build(),
            flow_box: FlowBox::builder()
                .max_children_per_line(4)
                .column_spacing(0)
                .row_spacing(0)
                .build(),
            set_button: button_builder("Apply"),
            src_button: button_builder("Source"),
            file_dialog: filed_builder(),
            swindow: ScrolledWindow::builder()
                .hexpand(true)
                .vexpand(true)
                .has_frame(true)
                .build(),
            vbox: vbox_builder(),
            main_window: ApplicationWindow::builder()
                .application(&app)
                .default_width(1250)
                .default_height(700)
                .resizable(false)
                .build(),
        };

        return main_window;
    }
}
