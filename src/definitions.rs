use std::{fs::File, io::Write, sync::MutexGuard};

pub static mut IMAGES: Vec<String> = Vec::<String>::new();
pub static mut SELECTED: i32 = 0;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Environment {
    pub hypr_sig: String,
    pub xdg_runtime_dir: String,
    pub xdg_config_dir: String,
    pub home: String,
}

impl Environment {
    pub fn init() -> Self {
        Self {
            hypr_sig: std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap(),
            xdg_runtime_dir: std::env::var("XDG_RUNTIME_DIR").unwrap(),
            xdg_config_dir: std::env::var("XDG_CONFIG_DIR").unwrap(),
            home: std::env::var("HOME").unwrap(),
        }
    }
}

pub fn update_config(e: MutexGuard<Environment>, src: String) {
    let config = format!("preload = {}\nwallpaper = ,{}", src, src);
    dbg!(format!("{}/.config/hypr/hyprpaper.conf", e.home));
    if let Ok(mut hyprconf) = File::create(format!("{}/.config/hypr/hyprpaper.conf", e.home)) {
        hyprconf
            .write(&config.into_bytes())
            .expect("Failed to write configuration.");
        drop(hyprconf);
        drop(e);
    }
}
