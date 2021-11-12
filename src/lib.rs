use actix_files::Files;
use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger;
use tool::{app_state::AppState, configure::Config, token::TokenTool};

mod app;
pub mod middlewares;
pub mod tool;

pub async fn start() -> std::io::Result<()> {
    let config = Data::new(Config::from_file("./config.json"));
    let port = config.port();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(config.log_level()));
    let app_state = Data::new(AppState::new("My Web"));
    let token_tool = Data::new(TokenTool::new(config.key()));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(config.clone())
            .app_data(token_tool.clone())
            .app_data(app_state.clone())
            .configure(app::config)
            .service(Files::new("/", config.static_dir()).show_files_listing())
            .default_service(
                web::route() // TODO: get 404->index
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::NotFound().body("404 Not Found!")),
            )
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
