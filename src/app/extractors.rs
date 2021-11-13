use actix_web::{
    get, post,
    web::{self, Json, Path, Query, ServiceConfig},
    HttpRequest,
};
use serde::Deserialize;

use crate::middlewares::sayhi::SayHi;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/extractoers")
            .service(extractors_path)
            .service(extractors_query)
            .service(extractors_json)
            .service(say_hi_midll)
            .wrap(SayHi),
    );
}

#[get("/say_hi_midll")]
async fn say_hi_midll(req: HttpRequest) -> String {
    let ex = req.extensions();
    let s: &String = ex.get().unwrap();
    format!("say_hi_midll, {}", s)
}

#[get("/path/{p1}/{p2}")]
async fn extractors_path(req: HttpRequest, Path((p1, p2)): Path<(String, u32)>) -> String {
    let key: String = req.match_info().query("p1").parse().unwrap();
    format!("p1= {}, p2= {}, key= {}", p1, p2, key)
}

#[derive(Deserialize)]
struct Q {
    name: String,
    age: u32,
}

#[get("/query")]
async fn extractors_query(q: Query<Q>) -> String {
    format!("q.name: {}, q.age: {}", q.name, q.age)
}

#[derive(Deserialize)]
struct JQ {
    name: Option<String>,
    age: u32,
}
#[post("/json")]
async fn extractors_json(q: Json<JQ>) -> String {
    // if let None = q.name {
    //     q.name.replace("value".to_string());
    // }
    let name = match &q.name {
        None => "none",
        Some(name) => name,
    };
    format!("q.name: {}, q.age: {}", name, q.age)
}
