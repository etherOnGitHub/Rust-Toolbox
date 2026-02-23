use crate::domain::Health;

pub fn build_banner(environment: &str) -> String {
    let health = Health::new("core", true);
    format!("Rust Toolbox ({environment}) [{}]", health.summary())
}
