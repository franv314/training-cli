/*  Copyright 2023 Francesco Vercellesi
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

use super::SUBMISSION_API_URL;
use crate::error;
use serde_json::{json, Value};

pub fn get_submissions_on_task(task: &str, token: &str) -> error::Result<Value> {
    let req = json!({
        "action": "list",
        "task_name": task,
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
        return error::Result::Err(error::Error::Api(
            "Failed to fetch submissions! ".to_string() + json.get("error").unwrap().as_str().unwrap(),
        ));
    }

    Ok(json)
}

pub fn get_submission_details(sub_id: i64, token: &str) -> error::Result<Value> {
    let req = json!({
        "action": "details",
        "id": sub_id,
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
        return error::Result::Err(error::Error::Api(
            "Failed to fetch submission! ".to_string() + json.get("error").unwrap().as_str().unwrap(),
        ));
    }

    Ok(json)
}
