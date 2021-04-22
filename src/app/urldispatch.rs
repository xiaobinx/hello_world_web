use actix_web::{
    get, guard,
    http::header,
    web::{self, ServiceConfig},
    HttpRequest, HttpResponse, Responder, Result,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/urldispatch")
            .service(urldispatch_test)
            .service(
                web::resource("/r")
                    .name("r")
                    .route(web::get().to(|| HttpResponse::Ok().body("get!")))
                    .route(web::post().to(|| HttpResponse::Ok().body("post!"))),
            )
            .service(
                // 不知道什么意思
                web::resource("/test2/{a}/{b}/{c}")
                    .name("foo") // <- 设置资源名, 然后它可以被 'url_for'使用
                    .guard(guard::Get())
                    .to(|| HttpResponse::Ok().body("urldispatch_test2...")),
            )
            .service(urldispatch_test2)
            .service(notallowed),
    );
}

#[get("/test1")]
async fn urldispatch_test() -> String {
    "shit".to_string()
}

#[get("/test2")]
async fn urldispatch_test2(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("foo", &["1", "2", "3"])?; // <- 为"foo"资源生成url
    println!("url: {}", url);
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[get("/normalization")]
async fn urldispatch_external_resources(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap(); // <- 为"foo"资源生成url
    println!("url: {}", url);
    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish())
}

#[get("/notallowed")]
async fn notallowed() -> impl Responder {
    println!("req notallowed.");
    let txt = r#"
    what a fock.
    this is noetallowed
  "#;
    HttpResponse::BadRequest().body(txt)
}
