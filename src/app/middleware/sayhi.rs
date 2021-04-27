use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SayHi;

impl<S, B> Transform<S> for SayHi
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SayHiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SayHiMiddleware { service })
    }
}

pub struct SayHiMiddleware<S> {
    service: S,
}

impl<S, B> Service for SayHiMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        // {
        //     if labels.is_empty() {
        //         labels.push("shit 1".to_string());
        //     }
        //     println!("{:?}", data.labels);
        // }
        let data: &Data<crate::app::state::AppState> = req.app_data().unwrap();
        let data = data.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let fut = self.service.call(req);
        Box::pin(async move {
            println!("{} {}.-----------------", method, path);
            {
                let mut labels = data.labels.lock().await;
                if labels.is_empty() {
                    labels.push("shit 1".to_string());
                }
            }

            let res = fut.await?;
            let headers = res.headers();
            for (name, value) in headers.iter() {
                println!("{}: {}", name.to_string(), value.to_str().unwrap());
            }
            Ok(res)
        })
    }
}
