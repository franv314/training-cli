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
mod ui;

use std::env;
use std::fs;

const TOKEN_FILE: &str = "/home/fve5/training-token.txt";

fn main() -> error::Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        return Err(error::Error::Generic("No command given!".to_string()));
    }

    if args[1] == "login" {
        return ui::login();
    }

    let token = fs::read_to_string(TOKEN_FILE).map_err(|_| "No token found! Login via `training-cli login`")?;

    if args[1] == "logout" {
        ui::logout()?;
    } else if args[1] == "submit" {
        if args.len() < 4 {
            println!("Usage: `training-cli submit [task_name] [file1] ...`");
            return Err(error::Error::Generic("Not enough arguments!".to_string()));
        }
        api::submit::submit(&args[2], &args[3..], &token)?;
    } else if args[1] == "list-sub" {
        if args.len() < 3 {
            println!("Usage: `training-cli list-sub [task-name] [optional: # of subs]`");
            return Err(error::Error::Generic("Not enough arguments!".to_string()));
        }

        let no = if args.len() == 3 {
            usize::MAX
        } else {
            args[3]
                .parse()
                .map_err(|_| "Number of submissions to show should be an integer!")?
        };

        let subs = api::get_submissions::get_submissions_on_task(&args[2], &token)?;
        ui::print_submissions(&subs, no);
    } else if args[1] == "sub-details" {
        if args.len() < 3 {
            println!("Usage: `training-cli sub-details [sub_id]`");
            return Err(error::Error::Generic("Not enough arguments!".to_string()));
        }

        let sub_id = args[2].parse().map_err(|_| "Submission id should be an integer!")?;

        let sub_details = api::get_submissions::get_submission_details(sub_id, &token)?;
        ui::print_submission_details(&sub_details);
    }

    Ok(())
}
