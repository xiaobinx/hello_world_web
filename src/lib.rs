use actix_files::Files;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use app::state::AppState;

mod app;
mod err;
mod json;
pub async fn start() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new("My Web"));
    HttpServer::new(move || {
        App::new()
            .wrap(app::middleware::SayHi)
            .app_data(app_state.clone())
            .configure(app::config)
            .service(web::scope("/json").configure(json::config))
            .service(web::scope("/err").configure(err::config))
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
