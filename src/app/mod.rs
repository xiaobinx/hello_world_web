use actix_files::NamedFile;
use actix_web::{get, web, Result};
mod err;
mod extractors;
mod json;
pub mod middleware;
mod session;
pub mod state;
mod stream;
mod test;
mod tool;
mod urldispatch;

pub fn config(cfg: &mut web::ServiceConfig) {
    extractors::config(cfg);
    state::config(cfg);
    urldispatch::config(cfg);
    err::config(cfg);
    json::config(cfg);
    test::config(cfg);
    session::config(cfg);
    stream::config(cfg);

    cfg.service(index);
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}
