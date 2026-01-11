use clap::Parser;
use structs::cli::*;
use crate::ui_core::*;
use crate::app::App;

mod ui_core;
mod components;
mod controllers;
mod model;
mod services;
mod structs;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;

    Ok(())
}

// fn args_handler(note_ctrl: &NoteController) {
//     let args = Cli::parse();
//     let args_note = get_args_cmd();

//     match args.cmd {
//         Some(Commands::Spawn) => note_ctrl.spawn_sample_rows(),
//         Some(Commands::Dir) => println!("{}", IOFile::default_dir_path().display()),
//         Some(Commands::List) | Some(Commands::Ls) => note_ctrl.print_current_notes(),
//         None => note_ctrl.save_daily_note(args_note),
//     }
// }
