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

use serde::Deserialize;

pub mod get_submissions;
pub mod get_task;
pub mod login;
pub mod submit;

const SUBMISSION_API_URL: &str = "https://training.olinfo.it/api/submission";
const TASK_API_URL: &str = "https://training.olinfo.it/api/task";
const USER_API_URL: &str = "https://training.olinfo.it/api/user";

#[derive(Deserialize)]
pub struct Testcase {
    pub idx: String,
    pub memory: Option<i64>,
    pub outcome: String,
    pub text: String,
    pub time: Option<f64>,
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

#[derive(Deserialize)]
pub struct SubmissionFormat {
    pub submission_format: Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TaskFetchResult {
    Success(SubmissionFormat),
    Insuccess { error: String },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SubmitResult {
    Success { },
    Insuccess { error: String },
}