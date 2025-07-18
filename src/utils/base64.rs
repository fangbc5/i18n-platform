use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encode(data: &str) -> String {
    STANDARD.encode(data.as_bytes())
}

pub fn decode(data: &str) -> Result<String, base64::DecodeError> {
    let data = STANDARD.decode(data.as_bytes())?;
    // 如果你确定内容是 UTF-8，可以再尝试 String::from_utf8()
    if let Ok(s) = String::from_utf8(data) {
        Ok(s)
    } else {
        Err(base64::DecodeError::InvalidPadding)
    }
}
