use log::{error, info, warn};

pub fn initialize() {
    pretty_env_logger::init_timed();
}

pub fn info(file_name: &str, fn_name: &str, message: &str) {
    info!("[{} - {}] {}", file_name, fn_name, message);
}

pub fn warn(file_name: &str, fn_name: &str, message: &str) {
    warn!("[{} - {}] {}", file_name, fn_name, message);
}

pub fn error(file_name: &str, fn_name: &str, message: &str) {
    error!("[{} - {}] {}", file_name, fn_name, message);
}
