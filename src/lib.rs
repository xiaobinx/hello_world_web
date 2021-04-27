use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    guard,
    middleware::{self, Logger},
    web, App, HttpResponse, HttpServer,
};
use app::state::AppState;
use env_logger;

mod app;

pub async fn start() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let app_state = web::Data::new(AppState::new("My Web"));
    let key: [u8; 32] = [
        188, 128, 191, 219, 111, 226, 135, 91, 108, 99, 6, 247, 207, 5, 183, 33, 15, 97, 170, 69,
        170, 66, 186, 110, 39, 144, 42, 234, 29, 50, 91, 254,
    ];
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .wrap(app::middleware::sayhi::SayHi)
            .wrap(CookieSession::signed(&key).secure(false))
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
