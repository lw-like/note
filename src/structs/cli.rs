use clap::{Parser, Subcommand};
use std::env;

pub fn get_args_cmd() -> String {
    let args: Vec<String> = env::args().collect();
    String::from(if args.len() > 1 { &args[1] } else { "" })
        .trim()
        .to_string()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, ignore_errors(true), trailing_var_arg = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "Displays notes storage directory")]
    Dir,
    #[command(about = "Display current notes - daily")]
    List,
    #[command(about = "Shorthand for list command")]
    Ls,
    #[command(about = "Spawns sample notes")]
    Spawn,
}
