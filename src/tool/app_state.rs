use futures::lock::Mutex;

pub struct AppState {
    pub app_name: String,
    pub labels: Mutex<Vec<String>>,
}

impl AppState {
    pub fn new(app_name: &str) -> AppState {
        AppState {
            app_name: app_name.to_string(),
            labels: Mutex::new(Vec::new()),
        }
    }
}
