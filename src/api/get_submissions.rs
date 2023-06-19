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

pub fn get_submissions_on_task(task: &str, token: &str) -> error::Result<SubmissionList> {
    let req = ApiQuery {
        action: "list",
        task_name: Some(task.to_string()),
        ..EMPTY_QUERY
    };

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
        ResultSubmissionList::Insuccess { error } => Err(error::Error::Api(format!("Failed to fetch submissions! {error}"))),
    }
}

pub fn get_submission_details(sub_id: i64, token: &str) -> error::Result<SubmissionInfo> {
    let req = ApiQuery {
        action: "details",
        id: Some(sub_id),
        ..EMPTY_QUERY
    };

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
        ResultSubmissionInfo::Insuccess { error } => Err(error::Error::Api(format!("Failed to fetch submission! {error}"))),
    }
}
