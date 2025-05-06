mod model;
mod services;
mod structs;

use model::note::*;
use services::cmd_service as cmd;
use services::message_service::MessagesService;
use structs::file_input::FileInput;
use structs::std_input::StdInput;
use structs::input::SourceReader;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Warsaw;
use std::env::current_exe;
use chrono_tz::Tz;

static NOTE_DIR: &str = "notes";
static DATE_FORMAT: &str = "%Y-%m-%d";

fn get_base_dir() -> String {
    current_exe().unwrap().parent().unwrap().parent().unwrap().to_str().unwrap().to_string()
}

fn get_app_base_dir() -> String {
    [String::from(get_base_dir()), String::from(NOTE_DIR)].join("\\").replace("\\", "/")
}

fn get_prepared_file_name() -> String {
    let prefix = get_date().format(DATE_FORMAT).to_string();
    prefix + "_note"
}

fn get_file_dir() -> String {
    [get_app_base_dir(), String::from(get_prepared_file_name())].join("/")
}

fn get_date() -> DateTime<Tz> {
    Utc::now().with_timezone(&Warsaw)
}

fn open_file() -> File {
    OpenOptions::new().append(true).create(true).open(get_file_dir()).unwrap()
}

fn write_note(note_line: String, mut file: File) -> bool {
    if note_line.len() == 0 {
        return false;
    }

    file.write_all((note_line.clone() + "\n").as_bytes())
        .expect(MessagesService::get_file_write_fail_text());
    MessagesService::print_save_success(&Note::parse(note_line));
    true
}

fn print_last_notes() -> bool {
    let fin: FileInput = FileInput::new(get_file_dir());
    let value = match fin.read::<String>() {
        Some(x) => x,
        None => {
            MessagesService::print_notes_not_found();
            String::new()
        }
    };

    println!("{}", value);
    true
}

fn save_note() -> bool {
    let input_note: String = cmd::get_args_cmd().trim().to_string();

    if input_note.len() == 0 {
        return false;
    }

    let note = Note::new(input_note);
    write_note(String::from(note.serialize()), open_file().try_clone().unwrap());
    true
}

fn handle_user_input() {
    let inp = StdInput;
    let line = match inp.read::<String>() {
        Some(val) => val,
        None => String::new()
    };

    if handle_args(cmd::get_cmd_type(line.clone())) {
        return;
    }

    let note = Note::new(line);
    write_note(note.serialize(), open_file().try_clone().unwrap());
}

fn spawn_rows() -> bool {
    let file = open_file();
    for i in 0..10 {
        write_note(Note::new(i.to_string()).serialize(), file.try_clone().unwrap());
    }

    true
}

fn handle_args(cmd_type: cmd::CmdType) -> bool {
    if match cmd_type {
        cmd::CmdType::List => print_last_notes(),
        cmd::CmdType::Note => save_note(),
        cmd::CmdType::SpawnRows => spawn_rows(),
    } {
        true
    } else {
        false
    }
}

fn create_required_data_structure() {
    create_dir_all(get_app_base_dir()).unwrap();
}

fn main() {
    create_required_data_structure();

    if handle_args(cmd::get_args_cmd_type()) {
        return;
    }

    MessagesService::print_initial_info();
    handle_user_input();
}
