use actix_web::{get, web::Bytes, Error, HttpResponse};
use futures::future::ok;
use futures::stream::once;

#[get("/stream")]
pub async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
