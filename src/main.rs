mod model;
mod services;
mod structs;
mod controllers;

use controllers::note_controller::NoteController;
use services::cmd_service::{self as cmd, get_args_cmd};
use services::message_service::MessagesService;
use structs::input_file::Readable;
use structs::input_std::InputStd;

fn handle_args(storage: &NoteController, value: &String) -> bool {
    let cmd_type = cmd::get_cmd_type(&value);

    match cmd_type {
        cmd::CmdType::List => storage.print_current_notes(),
        cmd::CmdType::Note => storage.save_daily_note(value),
        cmd::CmdType::SpawnRows => storage.spawn_sample_rows(),
        cmd::CmdType::Interactive => false,
    }
}

fn handle_user_input(storage: &NoteController) {
    let inp = InputStd;
    let note_value = match inp.read::<String>() {
        Some(val) => val,
        None => String::new()
    };

    handle_args(&storage, &note_value);
}

fn main() {
    let note_ctrl = NoteController::new();

    if handle_args(&note_ctrl, &get_args_cmd()) {
        return;
    }

    MessagesService::print_initial_info();
    handle_user_input(&note_ctrl);
}
