use std::fmt::{write, Display};
use std::{env::current_exe, path::{self, Path, PathBuf}};

use crate::services::message_service::MessagesService;

static NOTE_DIR: &str = "notes";

pub struct IOFile {
    pub file_path: String,
}

impl IOFile {
    pub fn new(path: &str) -> IOFile {
        IOFile {
            file_path: String::from(path)
        }
    }

    pub fn path_exist(&self) -> bool {
        self.get_base_dir().exists()
    }

    fn current_path(&self) -> PathBuf {
        current_exe().expect(MessagesService::get_no_dir_text())
    }

    fn current_path_str(&self) -> String {
        self.current_path().display().to_string()
    }

    pub fn get_base_dir(&self) -> PathBuf {
        let paths_str= self.current_path_str();

        path::absolute(Path::new(&(paths_str + "../../../" + NOTE_DIR))).expect(MessagesService::get_no_dir_text())
    }

    pub fn default_dir_path() -> PathBuf {
        IOFile::new("").get_base_dir()
    }

    pub fn create_file_path(&self, file: &String) -> PathBuf {
        let mut path = self.get_base_dir();
        path.push(file);
        path
    }

    pub fn normalized_path(&self) -> PathBuf {
        self.create_file_path(&self.file_path)
    }
}

impl Display for IOFile { fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write(f, format_args!("{}", self.normalized_path().display()))
    }
}
