mod model;
mod services;

use model::note::*;
use services::cmd_service as cmd;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;
use chrono::{DateTime, Utc};
use chrono_tz::Europe::Warsaw;
use std::env::current_exe;
use chrono_tz::Tz;
use colored::Colorize;

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_note(note_line: String, mut file: File) {
    if note_line.len() == 0 {
        return;
    }

    file.write_all((note_line.clone() + "\n").as_bytes()).expect("Unable to write to file");
    print!("Note saved: ");
    Note::parse(note_line).print();
}

fn print_info() {
    println!("\n  Type:");
    println!("      ls or list to list last notes");
    println!("      If command is not recognized note will be taken");
}
fn print_last_notes() -> bool {
    if let Ok(lines) = read_lines(get_file_dir()) {
        println!("\n {}", "---- NOTES:".blue().bold());
        let mut i = 0;
        for line in lines.map_while(Result::ok) {
            i += 1;
            Note::parse(line).print();
        }
        println!("{} {} \n", "---- TOTAL COUNT: ".blue().bold(), (i.to_string()).red());
    }
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
    let line: String = cmd::read_user_input_line();

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

    print_info();
    handle_user_input();
}
