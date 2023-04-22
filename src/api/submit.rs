use super::SUBMISSION_API_URL;
use crate::error;
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, Value};
use std::fs;

fn get_language(filename: &str) -> Result<String, &str> {
    let extension = filename.split(|c| c == '.').last().unwrap_or("");
    match extension {
        "cc" | "cpp" => Ok(String::from("C++17 / g++")),
        "c" => Ok(String::from("C11 / gcc")),
        "java" => Ok(String::from("Java / JDK")),
        "py" => Ok(String::from("Python 3 / CPython")),
        "pas" => Ok(String::from("Pascal / fpc")),
        _ => Err("Could not resolve source language!"),
    }
}

pub fn submit(task: &str, filename: &str, token: &str) -> error::Result<()> {
    let task_resp = super::get_task::get_task(task)?;
    let submission_format = task_resp
        .get("submission_format")
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .unwrap();

    let req = json!({
        "action": "new",
        "task_name": task,
        "files": {
            submission_format : {
                "data": general_purpose::STANDARD.encode(fs::read_to_string(filename)?.as_bytes()),
                "language": get_language(filename)?,
                "filename": filename,
            },
        },
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(SUBMISSION_API_URL)
        .header("Cookie", token)
        .header("Content-Type", "application/json")
        .json(&req)
        .send()?;

    let json: Value = resp.json()?;

    if json.get("success").unwrap().as_i64().unwrap() == 0 {
        return error::Result::Err(error::Error::ApiError(
            String::from("Failed to submit! ") + json.get("error").unwrap().as_str().unwrap(),
        ));
    }

    Ok(())
}
