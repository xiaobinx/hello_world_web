use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use serde::Serialize;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(jsontest);
}

#[derive(Serialize)]
struct MyJson {
    name: &'static str,
    age: u8,
    birth_date: u64,
}

#[get("/jsontest")]
async fn jsontest() -> impl Responder {
    HttpResponse::Ok().json(MyJson {
        name: "jack",
        age: 30,
        birth_date: 10000,
    })
}
