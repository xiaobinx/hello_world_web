use actix_web::get;
use std::thread;
#[get("/ThreadName")]
async fn thread_name() -> String {
    let thread_name = thread::current().id().clone();
    let body = format!("thread name: {:?}", thread_name);
    body
}
