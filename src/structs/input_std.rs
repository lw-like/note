use std::{fmt::Display, io};

use super::input_file::{format_input_display, InputBase, Readable, INPUT_SOURCE_NAME_STD};

pub struct InputStd;
impl Display for InputStd {fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { format_input_display(f, self) }}

impl InputBase for InputStd {
    fn source_name(&self) -> String {
        INPUT_SOURCE_NAME_STD.to_string()
    }
}

impl Readable for InputStd {
    fn open(&self) -> std::fs::File {
        panic!("InputStd doesnt support open method!")
    }

    fn read<T: From<String>>(&self) -> Option<T> {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        Some(T::from(line.trim().to_string()))
    }
}