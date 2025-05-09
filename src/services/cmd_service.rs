use std::env;

pub enum CmdType {
    Interactive,
    List,
    Note,
    SpawnRows, // Debug/Test purposes @todo: hide it for release builds
}

pub fn get_args_cmd() -> String {
    let args: Vec<String> = env::args().collect();
    String::from(if args.len() > 1 { &args[1] } else { "" })
        .trim()
        .to_string()
}

pub fn is_list_command(arg: &String) -> bool {
    arg == "ls" || arg == "list"
}

pub fn is_spawn_records_command(arg: &String) -> bool {
    arg == "spawn"
}

pub fn is_interactive(arg: &str) -> bool {
    arg.is_empty()
}

pub fn get_cmd_type(cmd: &String) -> CmdType {
    if is_list_command(cmd) {
        return CmdType::List;
    } else if is_spawn_records_command(cmd) {
        return CmdType::SpawnRows;
    } else if is_interactive(cmd) {
        return CmdType::Interactive;
    }

    CmdType::Note
}

