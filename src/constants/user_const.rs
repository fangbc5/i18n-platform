pub const USER_NAME_MIN_LENGTH: usize = 5;
pub const USER_NAME_MAX_LENGTH: usize = 32;
pub const USER_PASSWORD_MIN_LENGTH: usize = 8;
pub const USER_PASSWORD_MAX_LENGTH: usize = 32;
pub const USER_EMAIL_MIN_LENGTH: usize = 5;
pub const USER_EMAIL_MAX_LENGTH: usize = 32;
pub const USER_EMAIL_REGEX: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
pub const USER_PHONE_REGEX: &str = r"^1[3-9]\d{9}$";