use log::{debug, error, info, warn};

pub fn error(s: &str) {
    let signed = format!("[ERROR] {s}");
    error!("{s}");
}
pub fn warn(s: &str) {
    let signed = format!("[WARN]  {s}");
    warn!("{s}");
}
pub fn info(s: &str) {
    let signed = format!("[INFO]  {s}");
    info!("{s}");
}
pub fn debug(s: &str) {
    let signed = format!("[DEBUG] {s}");
    debug!("{s}");
}
fn output(s: String) {}
