use actix_web::{
    get,
    web::{Bytes, ServiceConfig},
    Error, HttpResponse,
};
use futures::future::ok;
use futures::stream::once;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(stream);
}

#[get("/stream")]
pub async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}
