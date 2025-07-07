use crate::{errors::AppError, middleware::auth::AuthenticatedUser, utils::casbin::CasbinService};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use axum::{
    extract::State,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct CasbinMiddleware {
    resource: String,
    action: String,
}

impl CasbinMiddleware {
    pub fn new(resource: &str, action: &str) -> Self {
        Self {
            resource: resource.to_string(),
            action: action.to_string(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CasbinMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CasbinMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CasbinMiddlewareService {
            service,
            resource: self.resource.clone(),
            action: self.action.clone(),
        }))
    }
}

pub struct CasbinMiddlewareService<S> {
    service: S,
    resource: String,
    action: String,
}

impl<S, B> Service<ServiceRequest> for CasbinMiddlewareService<S>
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
        let auth_user = req.extensions().get::<AuthenticatedUser>().cloned();
        let resource = self.resource.clone();
        let action = self.action.clone();
        let fut = self.service.call(req);

        Box::pin(async move {
            if let Some(auth_user) = auth_user {
                let mut casbin = CasbinService::new().await.map_err(|e| Error::from(e))?;
                let has_permission = casbin
                    .check_permission(&auth_user.user_id.to_string(), &resource, &action)
                    .await
                    .map_err(|e| Error::from(e))?;

                if !has_permission {
                    return Err(Error::from(AppError::AuthError(
                        "Permission denied".to_string(),
                    )));
                }
            }

            fut.await
        })
    }
}

pub async fn casbin_middleware<B>(
    State(state): State<Arc<crate::AppState>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let path = req.uri().path();
    let method = req.method();

    // 从请求扩展中获取用户信息
    let claims = req
        .extensions()
        .get::<crate::utils::jwt::Claims>()
        .ok_or_else(|| AppError::Auth("用户未认证".to_string()))?;

    // 检查用户权限
    let can_access = state
        .casbin
        .enforce((claims.sub.clone(), path, method.as_str()))
        .map_err(|e| AppError::Forbidden(e.to_string()))?;

    if !can_access {
        return Err(AppError::Forbidden("没有访问权限".to_string()));
    }

    Ok(next.run(req).await)
}

// Casbin策略定义
pub const CASBIN_MODEL: &str = r#"
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && keyMatch2(r.obj, p.obj) && (r.act == p.act || p.act == "*")
"#;

// 默认策略
pub const DEFAULT_POLICIES: &[(&str, &str, &str)] = &[
    // 超级管理员可以访问所有资源
    ("admin", "/*", "*"),
    // 项目管理
    ("project_manager", "/api/projects*", "*"),
    ("project_viewer", "/api/projects*", "GET"),
    // 翻译管理
    ("translator", "/api/translations*", "*"),
    ("reviewer", "/api/translations/review*", "*"),
    // 术语管理
    ("term_manager", "/api/terms*", "*"),
    ("term_viewer", "/api/terms*", "GET"),
    // 词条管理
    ("phrase_manager", "/api/phrases*", "*"),
    ("phrase_viewer", "/api/phrases*", "GET"),
];
