use std::{fmt::Display, io};
use super::input::{format_input_display, InputBase, SourceReader, INPUT_SOURCE_NAME_STD};

pub struct StdInput;
impl Display for StdInput {fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { format_input_display(f, self) }}

impl InputBase for StdInput {
    fn source_name(&self) -> String {
        INPUT_SOURCE_NAME_STD.to_string()
    }
}

impl SourceReader for StdInput {
    fn read<T: From<String>>(&self) -> Option<T> {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        Some(T::from(line.trim().to_string()))
    }
}