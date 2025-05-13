mod controllers;
mod model;
mod services;
mod structs;

use clap::Parser;
use controllers::note_controller::NoteController;
use structs::cli::*;
use structs::io_file::IOFile;

fn main() {
    let note_ctrl = NoteController::new();
    args_handler(&note_ctrl);
}

fn args_handler(note_ctrl: &NoteController) {
    let args = Cli::parse();
    let args_note = get_args_cmd();

    match args.cmd {
        Some(Commands::Spawn) => note_ctrl.spawn_sample_rows(),
        Some(Commands::Dir) => println!("{}", IOFile::default_dir_path().display()),
        Some(Commands::List) | Some(Commands::Ls) => note_ctrl.print_current_notes(),
        None => note_ctrl.save_daily_note(&args_note),
    }
}
