use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data, ServiceConfig},
    Result,
};
use futures::lock::Mutex;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/state")
            .service(get_app_name)
            .service(get_labels)
            .service(add_label),
    );
}

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

#[post("/add_label")]
async fn add_label(body: String, data: Data<AppState>) -> Result<String> {
    let mut labels = data.labels.lock().await;
    if body.is_empty() {
        Err(ErrorBadRequest("body is empty!"))
    } else {
        labels.push(body);
        Ok(format!("add succ.labels-> {:?}!", labels))
    }
}

#[get("/get_labels")]
async fn get_labels(data: Data<AppState>) -> String {
    let labels = &data.labels;
    format!("labels-> {:?}!", labels)
}

#[get("/get_app_name")]
async fn get_app_name(data: Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}
