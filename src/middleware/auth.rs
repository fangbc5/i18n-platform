use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::future::{ready, Ready};

use crate::errors::AppError;
use crate::utils::jwt::Claims;

pub struct Authentication;

impl Default for Authentication {
    fn default() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = match req.headers().get("Authorization") {
            Some(auth_header) => {
                let auth_str = auth_header
                    .to_str()
                    .map_err(|_| AppError::Unauthorized("Invalid Authorization header".into()));

                match auth_str {
                    Ok(str) => {
                        if !str.starts_with("Bearer ") {
                            return Box::pin(ready(Err(AppError::Unauthorized(
                                "Invalid Authorization header format".into(),
                            )
                            .into())));
                        }
                        str[7..].to_string()
                    }
                    Err(e) => return Box::pin(ready(Err(e.into()))),
                }
            }
            None => {
                return Box::pin(ready(Err(AppError::Unauthorized(
                    "Missing Authorization header".into(),
                )
                .into())))
            }
        };

        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let claims = match decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(token_data) => token_data.claims,
            Err(e) => return Box::pin(ready(Err(AppError::Unauthorized(e.to_string()).into()))),
        };

        req.extensions_mut().insert(claims);

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
