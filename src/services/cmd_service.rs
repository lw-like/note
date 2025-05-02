use std::{env, io};

pub enum CmdType {
    List,
    Note,
    SpawnRows, // Debug/Test purposes @todo: hide it for release builds
}

pub fn read_user_input_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}

pub fn get_args_cmd() -> String {
    let args: Vec<String> = env::args().collect();
    String::from( if args.len() > 1 { &args[1] } else { "" })
}

pub fn is_list_command(line: String) -> bool { line == "ls" || line == "list" }
pub fn is_spawn_records_command(line: String) -> bool { line == "spawn" }

pub fn get_cmd_type() -> CmdType {
    let cmd = get_args_cmd();
    if is_list_command(cmd.clone()) {
        return CmdType::List;
    } else if is_spawn_records_command(cmd.clone()) {
        return CmdType::SpawnRows;
    }

    CmdType::Note
}