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
use base64::{engine::general_purpose, Engine as _};
use serde_json::{json, to_value, Value};
use std::collections::HashMap;
use std::fs;

fn get_language(filename: &str) -> Result<String, &str> {
    let extension = filename.split(|c| c == '.').last().unwrap_or("");
    match extension {
        "cc" | "cpp" => Ok(String::from("C++17 / g++")),
        "c" => Ok(String::from("C11 / gcc")),
        "java" => Ok(String::from("Java / JDK")),
        "py" => Ok(String::from("Python 3 / CPython")),
        "pas" => Ok(String::from("Pascal / fpc")),
        "txt" => Ok(String::from("")),
        _ => Err("Could not resolve source language!"),
    }
}

pub fn submit(task: &str, filenames: &[String], token: &str) -> error::Result<()> {
    let task_resp = super::get_task::get_task(task)?;
    let submission_format = task_resp
        .get("submission_format")
        .unwrap()
        .as_array()
        .ok_or("Could not get submission format for this task!")?;

    if submission_format.len() > filenames.len() {
        return Err(error::Error::Generic(String::from(
            "Not enough files to submit!",
        )));
    }

    let files = to_value(HashMap::<&str, _>::from_iter(submission_format
        .iter()
        .enumerate()
        .map(|(i, file)| -> Result<_, error::Error> { Ok((
            file.as_str().unwrap(),
            json!({
                "data": general_purpose::STANDARD.encode(fs::read_to_string(&filenames[i])?.as_bytes()),
                "language": get_language(&filenames[i])?,
                "filename": filenames[i],
            }))
        )} ).collect::<Result<Vec<_>, _>>()?)).unwrap();

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

    let json: Value = resp.json()?;

    if json.get("success").unwrap().as_i64().unwrap() == 0 {
        return error::Result::Err(error::Error::Api(
            String::from("Failed to submit! ") + json.get("error").unwrap().as_str().unwrap(),
        ));
    }

    Ok(())
}
