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
mod cli;
mod ui;

use std::fs;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};

const TOKEN_FILE: &str = "/home/fve5/training-token";

fn get_token() -> Result<String> {
    fs::read_to_string(TOKEN_FILE).with_context(|| format!("Failed to fetch token file at {TOKEN_FILE}"))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login => ui::login(),
        Commands::Logout => ui::logout(),
        Commands::Submit(args) => {
            let token = get_token()?;
            api::submit::submit(&args.task_name, &args.files, &token)
        },
        Commands::ListSub(args) => {
            let token = get_token()?;
            let subs = api::get_submissions::get_submissions_on_task(&args.task_name, &token)?;
            Ok(ui::print_submissions(&subs, args.count.unwrap_or(usize::MAX)))
        },
        Commands::SubDetails(args) => {
            let token = get_token()?;
            let sub_details = api::get_submissions::get_submission_details(args.sub_id, &token)?;
            Ok(ui::print_submission_details(&sub_details))
        },
    }
}
