use hello_world_web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    hello_world_web::start().await
}
