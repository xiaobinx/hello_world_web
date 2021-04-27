use actix_web::web::ServiceConfig;
mod errtest;
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(errtest::internal_error)
        .service(errtest::bad_client_data)
        .service(errtest::time_out)
        .service(errtest::error_helpers);
}
