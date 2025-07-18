use actix_web::{cookie::Cookie, HttpResponse};
use serde::{Deserialize, Serialize};

/**
 * 响应码定义
 */
pub const SUCCESS_CODE: i32 = 200;
pub const SERVER_ERROR_CODE: i32 = 500;

/**
 * 响应消息定义
 */
pub const SUCCESS_MESSAGE: &str = "ok";
pub const FAILURE_MESSAGE: &str = "failure";

#[derive(Debug, Serialize, Deserialize)]
pub struct R<'a, T: Serialize> {
    pub code: i32,
    pub message: &'a str,
    pub data: Option<T>,
}

impl<'a, T: Serialize> R<'a, T> {
    pub fn ok(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SUCCESS_CODE, message: SUCCESS_MESSAGE, data: Some(data) })
    }
    
    pub fn ok_with_cookie(data: T, cookie: Cookie) -> HttpResponse {
        let mut response = HttpResponse::Ok();
        response.cookie(cookie);
        response.json(Self { code: SUCCESS_CODE, message: SUCCESS_MESSAGE, data: Some(data) })
    }

    pub fn failure(message: &'a str) -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SERVER_ERROR_CODE, message, data: None })
    }

    pub fn failure_with_message(message: &'a str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SERVER_ERROR_CODE, message, data: Some(data) })
    }

    pub fn failure_with_message_data(message: &'a str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SERVER_ERROR_CODE, message, data: Some(data) })
    }

    pub fn failure_with_code_message_data(code: i32, message: &'a str, data: T) -> HttpResponse {
        HttpResponse::Ok().json(Self { code, message, data: Some(data) })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageR<'a, T: Serialize> {
    pub code: i32,
    pub message: &'a str,
    pub data: Vec<T>,
    pub total: i64,
}

impl<'a, T: Serialize> PageR<'a, T> {
    pub fn ok(result: (Vec<T>, i64)) -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SUCCESS_CODE, message: SUCCESS_MESSAGE, data: result.0, total: result.1 })
    }

    pub fn failure() -> HttpResponse {
        HttpResponse::Ok().json(Self { code: SERVER_ERROR_CODE, message: FAILURE_MESSAGE, data: vec![], total: 0 })
    }
}

pub fn default_page() -> u32 {
    1
}

pub fn default_size() -> u32 {
    20
}