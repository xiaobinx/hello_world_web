use actix_web::{get, web::ServiceConfig, Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use std::future::{ready, Ready};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(jsontest);
}

#[derive(Serialize)]
struct JsonWrapper<T>
where
    T: Serialize,
{
    data: T,
}

impl<T> Responder for JsonWrapper<T>
where
    T: Serialize,
{
    type Error = Error;

    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self.data).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Serialize)]
struct MyJson {
    name: &'static str,
    age: u8,
    birth_date: u64,
}

#[get("/jsontest")]
async fn jsontest() -> impl Responder {
    JsonWrapper {
        data: MyJson {
            name: "jack",
            age: 30,
            birth_date: 10000,
        },
    }
}
