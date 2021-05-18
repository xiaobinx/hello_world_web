use actix_files::NamedFile;
use actix_web::{get, web, Result};
mod err;
mod extractors;
mod json;
pub mod state;
mod stream;
mod test;
mod tokentest;
mod urldispatch;

pub fn config(cfg: &mut web::ServiceConfig) {
    tokentest::config(cfg);
    extractors::config(cfg);
    state::config(cfg);
    urldispatch::config(cfg);
    err::config(cfg);
    json::config(cfg);
    test::config(cfg);
    stream::config(cfg);

    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}
