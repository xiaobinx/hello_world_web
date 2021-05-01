use actix_files::Files;
use actix_web::{
    guard,
    middleware::{self, Logger},
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use app::{state::AppState, tool::token::TokenTool};
use env_logger;

mod app;

pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_state = web::Data::new(AppState::new("My Web"));

    HttpServer::new(move || {
        let token_tool = TokenTool::new(b"d566cb10-aa55-11eb-bcbc-0242ac130002");
        App::new()
            .app_data(Data::new(token_tool))
            .wrap(Logger::default())
            // .wrap(app::middleware::sayhi::SayHi)
            // .wrap(
            //     CookieSession::signed(&key)
            //         .secure(false)
            //         .http_only(true)
            //         .expires_in(1800),
            // )
            .wrap(middleware::Compress::default())
            .app_data(app_state.clone())
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
