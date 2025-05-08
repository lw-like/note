use std::fs::File;
use crate::services::message_service::MessagesService;
use super::io_file::IOFile;
use std::io::Write;

pub trait Writable {
    fn open(&self) -> File;
    fn save(&self, data: String);
}

impl Writable for IOFile {
    fn open(&self) -> File {
        let iofile = self.create_file_path(&self.file_path);
        File::options().append(true).create(true).open(&iofile)
            .expect(MessagesService::get_file_open_fail_text())
    }

    fn save(&self, data: String) {
        self.open().write_all((data.clone() + "\n").as_bytes())
            .expect(MessagesService::get_file_write_fail_text());
    }
}