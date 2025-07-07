use crate::{
    config::SETTINGS,
    errors::AppError,
    middleware::casbin::{CASBIN_MODEL, DEFAULT_POLICIES},
};
use casbin::{
    CoreApi, DefaultModel, Enforcer, FileAdapter, MgmtApi, RbacApi, Result as CasbinResult,
};
use std::fs;
use std::path::Path;

pub async fn init_casbin() -> Result<Enforcer, AppError> {
    let e = Enforcer::new(CASBIN_MODEL, DEFAULT_POLICIES)
        .await
        .map_err(|e| AppError::Permission(format!("初始化 Casbin 失败: {}", e)))?;

    Ok(e)
}

pub async fn add_policy(
    enforcer: &mut Enforcer,
    sub: &str,
    obj: &str,
    act: &str,
) -> CasbinResult<bool> {
    enforcer
        .add_policy(vec![sub.to_string(), obj.to_string(), act.to_string()])
        .await
}

pub async fn remove_policy(
    enforcer: &mut Enforcer,
    sub: &str,
    obj: &str,
    act: &str,
) -> CasbinResult<bool> {
    enforcer
        .remove_policy(vec![sub.to_string(), obj.to_string(), act.to_string()])
        .await
}

pub async fn add_role_for_user(
    enforcer: &mut Enforcer,
    user: &str,
    role: &str,
) -> CasbinResult<bool> {
    enforcer.add_role_for_user(user, role, None).await
}

pub async fn delete_role_for_user(
    enforcer: &mut Enforcer,
    user: &str,
    role: &str,
) -> CasbinResult<bool> {
    enforcer.delete_role_for_user(user, role, None).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_casbin_enforcer() {
        let mut enforcer = init_casbin().await.unwrap();

        // 测试添加策略
        add_policy(&mut enforcer, "alice", "/api/test", "GET")
            .await
            .unwrap();
        assert!(enforcer.enforce(("alice", "/api/test", "GET")).unwrap());

        // 测试移除策略
        remove_policy(&mut enforcer, "alice", "/api/test", "GET")
            .await
            .unwrap();
        assert!(!enforcer.enforce(("alice", "/api/test", "GET")).unwrap());

        // 测试角色分配
        add_role_for_user(&mut enforcer, "alice", "admin")
            .await
            .unwrap();
        assert!(enforcer.has_role_for_user("alice", "admin", None));

        // 测试角色移除
        delete_role_for_user(&mut enforcer, "alice", "admin")
            .await
            .unwrap();
        assert!(!enforcer.has_role_for_user("alice", "admin", None));
    }
}
