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

use super::*;
use crate::error;
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize)]
struct File {
    data: String,
    filename: String,
    language: String,
}

fn get_language(filename: &str) -> Result<String, &str> {
    let extension = filename.split(|c| c == '.').last().unwrap_or("");
    match extension {
        "cc" | "cpp" => Ok("C++17 / g++".to_string()),
        "c" => Ok("C11 / gcc".to_string()),
        "java" => Ok("Java / JDK".to_string()),
        "py" => Ok("Python 3 / CPython".to_string()),
        "pas" => Ok("Pascal / fpc".to_string()),
        "txt" => Ok("".to_string()),
        _ => Err("Could not resolve source language!"),
    }
}

pub fn submit(task: &str, filenames: &[String], token: &str) -> error::Result<()> {
    let task_resp = super::get_task::get_task(task)?;
    let submission_format = task_resp.submission_format;

    if submission_format.len() > filenames.len() {
        return Err(error::Error::Generic("Not enough files to submit!".to_string()));
    }

    let files = submission_format
        .iter()
        .enumerate()
        .map(|(i, name)| -> error::Result<(&String, File)> {
            Ok((
                name,
                File {
                    data: general_purpose::STANDARD.encode(fs::read_to_string(&filenames[i])?.as_bytes()),
                    language: get_language(&filenames[i])?,
                    filename: filenames[i].clone(),
                },
            ))
        })
        .collect::<error::Result<HashMap<_, _>>>()?;

    let req = json!({
        "action": "new",
        "task_name": task,
        "files": files,
    });

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(SUBMISSION_API_URL)
        .header("Cookie", token)
        .header("Content-Type", "application/json")
        .json(&req)
        .send()?;

    let json: SubmitResult = resp.json()?;

    match json {
        SubmitResult::Success {} => Ok(()),
        SubmitResult::Insuccess { error } => Err(error::Error::Api(format!("Failed to submit! {error}"))),
    }
}
