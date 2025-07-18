use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    // 租户id
    pub tenant_id: u64,
    // 用户名
    pub username: Option<String>,
    // 密码
    pub password: String,
    // 邮箱
    pub email: Option<String>,
    // 手机号
    pub phone: Option<String>,
    // 真实姓名
    pub realname: Option<String>,
    // 身份证号
    pub id_card: Option<String>,
    // 昵称
    pub nickname: Option<String>,
    // 头像
    pub avatar: Option<String>,
    // 性别
    pub gender: Option<i8>,
    // 生日
    pub birthday: Option<NaiveDateTime>,
    // 状态
    pub status: i8,
    // 最后登录时间
    pub last_login: Option<NaiveDateTime>,
    // 创建人
    pub crt_by: String,
    // 创建时间
    pub crt_at: DateTime<Utc>,
    // 更新人
    pub upt_by: Option<String>,
    // 更新时间
    pub upt_at: DateTime<Utc>,
}
