use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(default = "crate::utils::default_page")]
    pub page: u32,
    #[serde(default = "crate::utils::default_size")]
    pub size: u32,
    // 查询key支持编码、名称搜索
    pub search_key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i32,
    pub size: i32,
    pub total_pages: i32,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            message: None,
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            message: Some(message),
            data: None,
        }
    }
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: i64, page: i32, size: i32) -> Self {
        let total_pages = ((total as f64) / (size as f64)).ceil() as i32;
        Self {
            items,
            total,
            page,
            size,
            total_pages,
        }
    }
}
