use crate::model::note::Note;
use crate::services::{date_service::DateService, message_service::MessagesService};
use crate::structs::input_file::Readable;
use crate::structs::{io_file::IOFile, output_file::Writable};
use std::fs::create_dir_all;

pub struct NoteController;

// @todo: Some methods need to be moved into the service layer.
impl NoteController {
    pub fn new() -> NoteController {
        NoteController::create_required_data_structure();
        NoteController {}
    }

    fn create_required_data_structure() {
        create_dir_all(IOFile::default_dir_path())
            .expect(MessagesService::get_init_storage_failure_text());
    }

    pub fn get_daily_file_name(&self) -> String {
        DateService::date_formated() + "_note"
    }

    pub fn get_iofile(&self) -> IOFile {
        let file_name = self.get_daily_file_name();
        IOFile::new(&file_name)
    }

    pub fn save_daily_note(&self, value: &String) -> bool {
        let note = Note::new(value.clone());
        let iofile = self.get_iofile();
        iofile.save(note.serialize());
        MessagesService::print_save_success(&note);
        true
    }

    pub fn spawn_sample_rows(&self) -> bool {
        for i in 0..10 {
            self.save_daily_note(&i.to_string());
        }
        true
    }

    pub fn print_current_notes(&self) -> bool {
        let iofile = self.get_iofile();
        let value = match iofile.read::<String>() {
            Some(val) => val,
            None => {
                MessagesService::print_notes_not_found();
                String::new()
            }
        };

        println!("{}", value);
        true
    }
}

