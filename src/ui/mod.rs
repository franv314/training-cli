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

use crate::{
    api::{self, SubmissionInfo, SubmissionList},
    error, TOKEN_FILE,
};
use colored::*;
use std::cmp;
use std::fs;
use std::io::{self, Write};

const BYTES_IN_KIBIBYTE: i64 = 1024;
const BYTES_IN_MEBIBYTE: i64 = 1048576;
const BYTES_IN_GIBIBYTE: i64 = 1073741824;

pub fn logout() -> error::Result<()> {
    fs::remove_file(TOKEN_FILE)?;
    Ok(())
}

pub fn login() -> error::Result<()> {
    let mut username = String::new();
    let mut password = String::new();

    print!("Username: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut username)?;
    username.pop();

    print!("Password: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut password)?;
    password.pop();

    let token = api::login::login(&username, &password)?;

    fs::write(TOKEN_FILE, token)?;

    println!("Token saved at {TOKEN_FILE}. Delete that file or run `training-cli logout` to remove it");
    Ok(())
}

pub fn print_submissions(subs: &SubmissionList, count: usize) {
    for sub in &subs.submissions[..cmp::min(count, subs.submissions.len())] {
        let outcome_string = if sub.compilation_outcome.is_none() {
            "Compilazione in corso".blue()
        } else if sub.compilation_outcome.as_ref().unwrap() == "fail" {
            "Compilazione fallita".red()
        } else if sub.evaluation_outcome.is_none() {
            "Valutazione in corso".blue()
        } else {
            let sub_score = sub.score.unwrap();
            let score = format!("{}/100", sub_score);
            if sub_score == 0. {
                score.red()
            } else if sub_score == 100. {
                score.green()
            } else {
                score.yellow()
            }
        };

        println!("{:>7} {}", sub.id, outcome_string);
    }
}

pub fn print_submission_details(details: &SubmissionInfo) {
    let Some(compilation_outcome) = &details.compilation_outcome else {
        println!("{}", "Compilazione in corso".blue());
        return;
    };

    let Some(evaluation_outcome) = &details.evaluation_outcome else {
        println!("{}", "Valutazione in corso".blue());
        return;
    };

    if compilation_outcome != "ok" {
        println!("{}", "Compilazione fallita".red());
    } else if evaluation_outcome != "ok" {
        println!("{}", "Valutazione fallita".red());
    } else {
        let score = details.score.unwrap();
        let prnt = format!("{}/100", score);
        if score == 0. {
            println!("{}", prnt.red());
        } else if score == 100. {
            println!("{}", prnt.green());
        } else {
            println!("{}", prnt.yellow());
        }

        for subtask in &details.score_details {
            let idx = subtask.idx.unwrap_or(0);
            let max_score = subtask.max_score;
            let score = subtask
                .score
                .unwrap_or_else(|| max_score as f64 * subtask.score_fraction.unwrap());

            println!("Subtask {}: {:>6.2} / {:>6.2}", idx, score, max_score);

            for testcase in &subtask.testcases {
                let idx: i64 = testcase.idx.parse().unwrap();
                let memory = memory_string(testcase.memory.unwrap_or(0));
                let time = testcase.time.unwrap_or(0.);

                let text = match testcase.outcome.as_str() {
                    "Correct" => testcase.text.green(),
                    "Partially correct" => testcase.text.yellow(),
                    "Not correct" => testcase.text.red(),
                    x => panic!("{} outcome should not exist", x),
                };

                println!("{:>3}: {:>6.3} s {} {}", idx, time, memory, text);
            }
        }
    }
}

fn memory_string(memory: i64) -> String {
    if memory < BYTES_IN_MEBIBYTE {
        format!("{:>5.1} kiB", memory as f64 / BYTES_IN_KIBIBYTE as f64)
    } else if memory < BYTES_IN_GIBIBYTE {
        format!("{:>5.1} MiB", memory as f64 / BYTES_IN_MEBIBYTE as f64)
    } else {
        format!("{:>5.1} GiB", memory as f64 / BYTES_IN_GIBIBYTE as f64)
    }
}
