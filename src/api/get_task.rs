use super::TASK_API_URL;
use crate::error;
use serde_json::{json, Value};

pub fn get_task(task: &str) -> error::Result<Value> {
    let req = json!({
        "action": "get",
        "name": task,
    });

    let client = reqwest::blocking::Client::new();
    let resp = client.post(TASK_API_URL).json(&req).send()?;

    Ok(resp.json()?)
}
