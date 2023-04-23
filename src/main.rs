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

mod api;
mod error;

use std::env;
use std::fs;
use std::io::{self, Write};

const TOKEN_FILE: &str = "INSERISCI IL PERCORSO COMPLETO";

fn logout() -> error::Result<()> {
    fs::remove_file(TOKEN_FILE)?;
    Ok(())
}

fn login() -> error::Result<()> {
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

fn main() -> error::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args[1] == "logout" {
        return logout();
    }

    if args[1] == "login" {
        return login();
    }

    let token = fs::read_to_string(TOKEN_FILE)
        .map_err(|_| "No token found! Login via `training-cli login`")?;

    if args[1] == "submit" {
        if args.len() < 4 {
            println!("Usage: `training-cli submit [task_name] [file1] ...`");
            return Err(error::Error::Generic(String::from("Not enough arguments!")));
        }
        api::submit::submit(&args[2], &args[3..], &token)?;
    }

    Ok(())
}
