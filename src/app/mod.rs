use actix_web::{get, post, web, Either, Error, HttpResponse, Responder};
mod echo;
mod extractors;
pub mod middleware;
pub mod state;
mod stream;
mod thread_name;
mod urldispatch;
pub fn config(cfg: &mut web::ServiceConfig) {
    extractors::config(cfg);
    state::config(cfg);
    urldispatch::config(cfg);
    cfg.service(echo::echo)
        .service(thread_name::thread_name)
        .service(stream::stream)
        .service(either)
        .service(index);
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
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
