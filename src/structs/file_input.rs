use std::path::Path;
use std::{fmt::Display, fs::File};
use std::io::{BufReader, Read};
use crate::services::message_service::MessagesService;
use super::input::{format_input_display, InputBase, SourceReader, INPUT_SOURCE_NAME_FILE};

pub struct FileInput {
    pub path: String,
}

impl FileInput {
    pub fn new(path: String) -> FileInput {
        FileInput {
            path
        }
    }

    pub fn path_exist(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub fn open(path: &String) -> File {
        File::open(path).expect(MessagesService::get_file_read_fail_text())
    }
}

impl Display for FileInput { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { format_input_display(f, self) } }

impl InputBase for FileInput {
    fn source_name(&self) -> String {
        INPUT_SOURCE_NAME_FILE.to_string()
    }
}

impl SourceReader for FileInput {
    fn read<T: From<String>>(&self) -> Option<T> {
        if !self.path_exist() {
            return None;
        }

        let mut output = String::new();
        BufReader::new(FileInput::open(&self.path))
            .read_to_string(&mut output)
            .expect(MessagesService::get_file_read_fail_text());

        Some(T::from(output))
    }
}
