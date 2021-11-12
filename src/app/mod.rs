use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, Data},
    Result,
};

use crate::tool::configure::Config;
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
pub async fn index(config: Data<Config>) -> Result<NamedFile> {
    Ok(NamedFile::open(format!(
        "{}/index.html",
        config.static_dir()
    ))?)
}
