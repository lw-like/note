use std::{fmt::{write, Display, Formatter, Result}, fs::File, io::{BufReader, Read}};
use crate::services::message_service::MessagesService;

use super::io_file::IOFile;

pub static INPUT_SOURCE_NAME_STD: &str = "std_input";
pub static INPUT_SOURCE_NAME_FILE: &str = "file_input";

pub fn format_input_display<T: InputBase>(f: &mut Formatter, input: &T) -> Result {
    write(f, format_args!("{}", input.source_name()))
}

pub trait InputBase: Display {
    fn source_name(&self) -> String;
}

impl InputBase for IOFile {
    fn source_name(&self) -> String {
        INPUT_SOURCE_NAME_FILE.to_string()
    }
}

pub trait Readable {
    fn open(&self) -> File;
    fn read<T: From<String>>(&self) -> Option<T>;
}

impl Readable for IOFile {
    fn open(&self) -> File {
        // @todo: Implement better error handling in order to provide better user experience.
        // Now if there are no current notes - file doesn't exists - app shows MessagesService::get_file_open_fail_text()
        // and Error object - Os { code, kind, message}
        File::open(self.normalized_path()).expect(MessagesService::get_file_open_fail_text())
    }
    fn read<T: From<String>>(&self) -> Option<T> {
        if !self.path_exist() {
            return None;
        }

        let mut output = String::new();
        BufReader::new(self.open())
            .read_to_string(&mut output)
            .expect(MessagesService::get_file_open_fail_text());

        Some(T::from(output))
    }
}