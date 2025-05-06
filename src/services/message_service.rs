pub struct MessagesService;

impl MessagesService {
    pub fn print_initial_info() {
        println!("\n  Type:");
        println!("      ls or list to list last notes");
        println!("      If command is not recognized note will be taken");
    }

    pub fn print_save_success<T: ToString>(data: &T) {
        print!("Note saved: {}", data.to_string());
    }

    pub fn print_notes_not_found() {
        println!("Notes not found!");
    }

    pub fn get_file_write_fail_text() -> &'static str {
        "Unable to write file"
    }

    pub fn get_file_read_fail_text() -> &'static str {
        "Cannot open file!"
    }
}