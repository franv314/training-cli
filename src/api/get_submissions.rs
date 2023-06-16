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
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Testcase {
    pub idx: String,
    pub memory: i64,
    pub outcome: String,
    pub text: String,
    pub time: f64,
}

#[derive(Deserialize)]
pub struct ScoreDetails {
    pub idx: Option<i32>,
    pub max_score: i32,
    pub score_fraction: Option<f64>,
    pub score: Option<f64>,
    pub testcases: Vec<Testcase>,
}

#[derive(Deserialize)]
pub struct SubmissionInfo {
    pub score: Option<f64>,
    pub compilation_outcome: Option<String>,
    pub evaluation_outcome: Option<String>,
    pub score_details: Vec<ScoreDetails>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ResultSubmissionInfo {
    Success(SubmissionInfo),
    Insuccess { error: String },
}

#[derive(Deserialize)]
pub struct Submission {
    pub compilation_outcome: Option<String>,
    pub evaluation_outcome: Option<String>,
    pub id: i32,
    pub score: f64,
}

#[derive(Deserialize)]
pub struct SubmissionsOnTask {
    pub submissions: Vec<Submission>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ResultSubmissionList {
    Success(SubmissionsOnTask),
    Insuccess { error: String },
}

pub fn get_submissions_on_task(task: &str, token: &str) -> error::Result<SubmissionsOnTask> {
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

    let json: ResultSubmissionList = resp.json()?;

    match json {
        ResultSubmissionList::Success(x) => Ok(x),
        ResultSubmissionList::Insuccess { error } => Err(error::Error::Api(format!("Failed to fetch submissions! {}", error))),
    }
}

pub fn get_submission_details(sub_id: i64, token: &str) -> error::Result<SubmissionInfo> {
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

    let json: ResultSubmissionInfo = resp.json()?;

    match json {
        ResultSubmissionInfo::Success(x) => Ok(x),
        ResultSubmissionInfo::Insuccess { error } => Err(error::Error::Api(format!("Failed to fetch submission! {}", error))),
    }
}
