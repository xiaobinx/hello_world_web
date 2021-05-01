use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    Either, Error, HttpResponse, Responder, Result,
};
use rand::Rng;
use std::thread;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(either)
        .service(echo)
        .service(token_key_gen)
        .service(thread_name);
}

#[get("/token_key_gen")]
async fn token_key_gen() -> impl Responder {
    Json(rand::thread_rng().gen::<[u8; 32]>())
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
