use std::fmt::{write, Display, Formatter, Result};

pub trait InputBase: Display {
    fn source_name(&self) -> String;
}

pub trait SourceReader {
    fn read<T: From<String>>(&self) -> Option<T>;
}

pub fn format_input_display<T: InputBase>(f: &mut Formatter, input: &T) -> Result {
    write(f, format_args!("{}", input.source_name()))
}
