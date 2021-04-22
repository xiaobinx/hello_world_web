use actix_web::{
    dev::HttpResponseBuilder,
    error::ErrorBadRequest,
    get,
    http::{header, StatusCode},
    HttpResponse, ResponseError, Result,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/internal_error")]
async fn internal_error() -> Result<&'static str, MyError> {
    Err(MyError::InternalError)
}

#[get("/bad_client_data")]
async fn bad_client_data() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}

#[get("/time_out")]
async fn time_out() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

#[derive(Debug)]
struct MyError1 {
    name: &'static str,
}

// 不知道什麼意思
#[get("/error_helpers")]
async fn error_helpers() -> Result<&'static str> {
    let result: Result<&'static str, MyError1> = Err(MyError1 {
        name: "test error helpers",
    });
    Ok(result.map_err(|e| ErrorBadRequest(e.name))?)
}
