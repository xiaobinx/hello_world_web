use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpRequest, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Login;

impl<S, B> Transform<S> for Login
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoginMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoginMiddleware { service })
    }
}

pub struct LoginMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoginMiddleware<S>
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
        let method = req.method().to_string();
        let path = req.path().to_string();
        let mut s = &self.service;
        Box::pin(async move {
            println!("{} {}.-----------------", method, path);
            let fut = s.call(req);

            let res = fut.await?;
            // let new_res = ServiceResponse::new(req, HttpResponse::Ok().body("adf").into_body());
            Ok(res)
        })
    }
}
