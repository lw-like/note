mod controllers;
mod model;
mod services;
mod structs;

use controllers::note_controller::NoteController;
use services::cmd_service::{self as cmd, get_args_cmd};
use services::message_service::MessagesService;
use structs::input_file::Readable;
use structs::input_std::InputStd;

fn handle_args(note_ctrl: &NoteController, value: String) -> bool {
    let cmd_type = cmd::get_cmd_type(&value);

    match cmd_type {
        cmd::CmdType::List => note_ctrl.print_current_notes(),
        cmd::CmdType::Note => note_ctrl.save_daily_note(&value),
        cmd::CmdType::SpawnRows => note_ctrl.spawn_sample_rows(),
        cmd::CmdType::Interactive => false,
    }
}

fn handle_user_input(note_ctrl: &NoteController) {
    let inp = InputStd;
    let note_value = inp.read::<String>().unwrap_or_default();

    handle_args(note_ctrl, note_value);
}

fn main() {
    let note_ctrl = NoteController::new();

    if handle_args(&note_ctrl, get_args_cmd()) {
        return;
    }

    MessagesService::print_initial_info();
    handle_user_input(&note_ctrl);
}
