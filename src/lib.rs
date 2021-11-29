use actix_files::Files;
use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger;
use tool::{app_state::AppState, configure::Config, token::TokenTool};

use crate::app::spa_index;

mod app;
pub mod middlewares;
pub mod tool;

pub async fn start() -> std::io::Result<()> {
    let config = Data::new(Config::from_file("./config.json"));
    let _config = Data::clone(&config);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level()));
    let app_state = Data::new(AppState::new("My Web"));
    let token_tool = Data::new(TokenTool::new(config.key()));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&config))
            .app_data(Data::clone(&token_tool))
            .app_data(Data::clone(&app_state))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .configure(app::config)
            .service(Files::new("/", config.static_dir()).index_file("index.html"))
            .default_service(
                web::resource("")
                    .route(web::get().to(spa_index))
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::NotFound().body("404 Not Found!")),
            )
    })
    .bind(format!("{}:{}", _config.listen(), _config.port()))?
    .run()
    .await
}
