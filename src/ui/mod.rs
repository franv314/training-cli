use crate::{api, error, TOKEN_FILE};
use colored::*;
use serde_json::Value;
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

pub fn print_submissions(subs: &Value, count: usize) {
    let subs = subs.get("submissions").unwrap().as_array().unwrap();
    for sub in &subs[..cmp::min(count, subs.len())] {
        let id = sub.get("id").unwrap().as_i64().unwrap();
        let compilation_outcome = sub.get("compilation_outcome").unwrap();

        if compilation_outcome == &Value::Null {
            println!("{:>7} {}", id, "Compilazione in corso".blue());
        } else if compilation_outcome == &Value::String("fail".to_string()) {
            println!("{:>7} {}", id, "Compilazione fallita".red());
        } else if sub.get("evaluation_outcome").unwrap() == &Value::Null {
            println!("{:>7} {}", id, "Valutazione in corso".blue());
        } else {
            let score = sub.get("score").unwrap().as_f64().unwrap();
            let prnt = format!("{}/100", score);
            if score == 0. {
                println!("{:>7} {}", id, prnt.red());
            } else if score == 100. {
                println!("{:>7} {}", id, prnt.green());
            } else {
                println!("{:>7} {}", id, prnt.yellow());
            }
        }
    }
}

pub fn print_submission_details(details: &Value) {
    let compilation_outcome = details.get("compilation_outcome").unwrap();
    let evaluation_outcome = details.get("evaluation_outcome").unwrap();

    if compilation_outcome == &Value::Null {
        println!("{}", "Compilazione in corso".blue());
    } else if compilation_outcome != &Value::String("ok".to_string()) {
        println!("{}", "Compilazione fallita".red());
    } else if evaluation_outcome == &Value::Null {
        println!("{}", "Valutazione in corso".blue());
    } else if evaluation_outcome != &Value::String("ok".to_string()) {
        println!("{}", "Valutazione fallita".red());
    } else {
        let score = details.get("score").unwrap().as_f64().unwrap();
        let prnt = format!("{}/100", score);
        if score == 0. {
            println!("{}", prnt.red());
        } else if score == 100. {
            println!("{}", prnt.green());
        } else {
            println!("{}", prnt.yellow());
        }

        let subtasks = details.get("score_details").unwrap().as_array().unwrap();
        for subtask in subtasks {
            let idx = match subtask.get("idx") {
                Some(x) => x.as_i64().unwrap(),
                None => 0,
            };
            let max_score = subtask.get("max_score").unwrap().as_f64().unwrap();
            let score = match subtask.get("score") {
                Some(x) => x.as_f64().unwrap(),
                None => max_score * subtask.get("score_fraction").unwrap().as_f64().unwrap(),
            };

            println!("Subtask {}: {:>6.2} / {:>6.2}", idx, score, max_score);

            let testcases = subtask.get("testcases").unwrap().as_array().unwrap();
            for testcase in testcases {
                let idx: i64 = testcase.get("idx").unwrap().as_str().unwrap().parse().unwrap();
                let memory = testcase.get("memory").unwrap().as_i64().unwrap();
                let outcome = testcase.get("outcome").unwrap().as_str().unwrap();
                let text = testcase.get("text").unwrap().as_str().unwrap();
                let time = testcase.get("time").unwrap().as_f64().unwrap();

                if outcome == "Correct" {
                    println!("{:>3}: {:>6.3} s {} {}", idx, time, memory_string(memory), text.green());
                } else if outcome == "Partially correct" {
                    println!("{:>3}: {:>6.3} s {} {}", idx, time, memory_string(memory), text.yellow());
                } else {
                    println!("{:>3}: {:>6.3} s {} {}", idx, time, memory_string(memory), text.red());
                }
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
