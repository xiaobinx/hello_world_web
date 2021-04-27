use actix_files::Files;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use app::state::AppState;

mod app;
pub async fn start() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new("My Web"));
    HttpServer::new(move || {
        App::new()
            .wrap(app::middleware::login::Login)
            .wrap(app::middleware::sayhi::SayHi)
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
