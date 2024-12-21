use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(version = "0.1.0", name = "training-cli")]
#[command(about = "Submit on the Italian OI training website", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Open login interaction
    Login,
    /// Delete token file
    Logout,
    /// Make a submission
    Submit(SubmitArgs),
    /// List submissions on a task
    ListSub(ListSubArgs),
    /// Show the details for a submission
    SubDetails(SubDetailsArgs),
}

#[derive(Args)]
pub struct SubmitArgs {
    /// The short name of the task to submit on
    pub task_name: String,
    /// The files to submit
    pub files: Vec<String>,
}

#[derive(Args)]
pub struct ListSubArgs {
    /// The short name of the task
    pub task_name: String,
    /// How many subs to show, all if unspecified
    pub count: Option<usize>,
}

#[derive(Args)]
pub struct SubDetailsArgs {
    /// The ID of the sub
    pub sub_id: i64,
}
