use actix_web::{get, post, web::ServiceConfig, Either, Error, HttpResponse, Responder, Result};
use std::thread;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(either).service(echo).service(thread_name);
}

#[get("/ThreadName")]
async fn thread_name() -> String {
    let thread_name = thread::current().id().clone();
    let body = format!("thread name: {:?}", thread_name);
    body
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

type EitherResult = Either<HttpResponse, Result<String, Error>>;

#[post("/either")]
async fn either(body: String) -> EitherResult {
    if body.is_empty() {
        Either::A(HttpResponse::BadRequest().body("Bad Data!"))
    } else {
        Either::B(Ok(body))
    }
}
