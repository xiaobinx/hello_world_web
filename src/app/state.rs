use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder, Result,
};

use crate::tool::app_state::AppState;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/state_test")
            .service(index)
            .service(append_label),
    );
}

#[get("")]
async fn index(state: Data<AppState>) -> impl Responder {
    let labels = state.labels.lock().await;
    HttpResponse::Ok().json(&*labels)
}

#[post("/append_label")]
async fn append_label(state: Data<AppState>, body: String) -> Result<String> {
    if body.is_empty() {
        Err(ErrorBadRequest("body is empty!"))
    } else {
        let mut labels = state.labels.lock().await;
        labels.push(body);
        Ok(format!("add succ.labels-> {:?}!", &*labels))
    }
}
