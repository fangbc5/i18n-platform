use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, Ready};
use std::future::Future;
use std::pin::Pin;

use crate::{errors::AppError, utils::R};

pub struct ErrorHandler;

impl Default for ErrorHandler {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for ErrorHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = ErrorMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ErrorMiddleware { service }))
    }
}

pub struct ErrorMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ErrorMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => Ok(res),
                Err(e) => {
                    let err_msg = if let Some(app_err) = e.as_error::<AppError>() {
                        app_err.to_string()
                    } else {
                        "Internal Server Error".to_string()
                    };
                    let (req, _) = req.into_parts();
                    Ok(ServiceResponse::new(req, R::<String>::failure(&err_msg)))
                }
            }
        })
    }
}
