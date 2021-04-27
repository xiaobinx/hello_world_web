use super::tool::{check_user, User};
use actix_session::Session;
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
    HttpResponse, Responder, Result,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(auth_get).service(auth_post);
}

#[get("/auth_get")]
async fn auth_get(session: Session) -> Result<impl Responder> {
    let user = check_user(&session)?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/auth_post")]
async fn auth_post(user_data: Json<User>, session: Session) -> Result<impl Responder> {
    let user = user_data.into_inner();
    session.set("user", user.clone())?;
    Ok(HttpResponse::Ok().json(user))
}
