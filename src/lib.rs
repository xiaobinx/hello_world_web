use actix_files::Files;
use actix_web::{
    guard,
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use env_logger;
use tool::{app_state::AppState, token::TokenTool};

mod app;
pub mod middlewares;
pub mod tool;

pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_state = Data::new(AppState::new("My Web"));
    let token_tool = Data::new(TokenTool::new(b"d566cb10-aa55-11eb-bcbc-0242ac130002"));
    HttpServer::new(move || {
        App::new()
            // .wrap(app::middleware::sayhi::SayHi)
            // .wrap(
            //     CookieSession::signed(&key)
            //         .secure(false)
            //         .http_only(true)
            //         .expires_in(1800),
            // )
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(app_state.clone())
            .app_data(token_tool.clone())
            .configure(app::config)
            .service(Files::new("/", "./static").show_files_listing())
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(|| HttpResponse::NotFound().body("404 Not Found!")),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
