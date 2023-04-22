use super::USER_API_URL;
use crate::error;
use serde_json::json;

pub fn login(username: &str, password: &str) -> error::Result<String> {
    let req = json!({
        "action": "login",
        "keep_signed": "true",
        "username": username,
        "password": password,
    });

    let client = reqwest::blocking::Client::new();
    let resp = client.post(USER_API_URL).json(&req).send()?;

    let token = resp
        .headers()
        .get("set-cookie")
        .ok_or(error::Error::ApiError(String::from("Failed to login!")))?;

    Ok(String::from(token.to_str().unwrap()))
}
