use gtk4::{prelude::*, Box, Button};

pub trait Sizable {
    fn default_size(&self, xsize: i32, ysize: i32);
}

impl Sizable for Button {
    fn default_size(&self, xsize: i32, ysize: i32) {
        self.set_width_request(xsize);
        self.set_height_request(ysize);
    }
}

impl Sizable for Box {
    fn default_size(&self, xsize: i32, ysize: i32) {
        self.set_width_request(xsize);
        self.set_height_request(ysize);
    }
}
