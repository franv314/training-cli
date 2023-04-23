use crate::{api, error, TOKEN_FILE};
use colored::*;
use serde_json::Value;
use std::cmp;
use std::fs;
use std::io::{self, Write};

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

    println!(
        "Token saved at {TOKEN_FILE}. Delete that file or run `training-cli logout` to remove it"
    );
    Ok(())
}

pub fn print_submissions(subs: &Value, count: usize) {
    let subs = subs.get("submissions").unwrap().as_array().unwrap();
    dbg!(subs);
    for sub in &subs[..cmp::min(count, subs.len())] {
        let compilation_outcome = sub.get("compilation_outcome").unwrap();
        if compilation_outcome == &Value::Null {
            println!("{}", "Compilazione in corso".blue());
        } else if compilation_outcome == &Value::String("fail".to_string()) {
            println!("{}", "Compilazione fallita".red());
        } else if sub.get("evaluation_outcome").unwrap() == &Value::Null {
            println!("{}", "Valutazione in corso".blue());
        } else {
            let score = sub.get("score").unwrap().as_f64().unwrap();
            let prnt = format!("{}/100", score);
            if score == 0. {
                println!("{}", prnt.red());
            } else if score == 100. {
                println!("{}", prnt.green());
            } else {
                println!("{}", prnt.yellow());
            }
        }
    }
}
