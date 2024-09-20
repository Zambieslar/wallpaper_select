use gtk4::Image;

pub static mut IMAGES: Vec<String> = Vec::<String>::new();
pub static mut SELECTED: i32 = 0;

#[derive(Debug, Clone)]
pub struct Environment {
    pub hypr_sig: String,
    pub xdg_runtime_dir: String,
    pub home: String,
}

impl Environment {
    pub fn init() -> Self {
        Self {
            hypr_sig: std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap(),
            xdg_runtime_dir: std::env::var("XDG_RUNTIME_DIR").unwrap(),
            home: std::env::var("HOME").unwrap(),
        }
    }
}
