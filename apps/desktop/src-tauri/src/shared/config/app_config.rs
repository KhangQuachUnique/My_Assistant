#[derive(Debug, Clone)]
pub struct AppConfig {
    pub application_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            application_name: String::from("AI Desktop Assistant"),
        }
    }
}
