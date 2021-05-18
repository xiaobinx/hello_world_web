use actix_web::{
    get, post,
    web::{self, Data, Json, ServiceConfig},
    HttpRequest, HttpResponse, Result,
};

use crate::tool::token::{ArcTokenTool, TokenInfo};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/token_test")
            .service(gen_token)
            .service(get_token),
    );
}

#[post("/gen_token")]
async fn gen_token(tool: Data<ArcTokenTool>, token_info: Json<TokenInfo>) -> Result<String> {
    Ok(tool.sign(&token_info)?)
}

#[get("/get_token")]
async fn get_token(tool: Data<ArcTokenTool>, req: HttpRequest) -> Result<HttpResponse> {
    let info = tool.verify_from_req(&req)?;
    Ok(HttpResponse::Ok().json(info))
}
