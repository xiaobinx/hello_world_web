use super::tool::token::{TokenInfo, TokenTool};
use actix_web::{
    get, post,
    web::{Data, Json, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(gen_token).service(get_token);
}

#[post("/gen_token")]
async fn gen_token(tool: Data<TokenTool>, token_info: Json<TokenInfo>) -> Result<String> {
    Ok(tool.sign(&token_info)?)
}

#[get("/get_token")]
async fn get_token(tool: Data<TokenTool>, req: HttpRequest) -> Result<HttpResponse> {
    let info = tool.verify_from_req(&req)?;
    Ok(HttpResponse::Ok().json(info))
}
