use crate::get_date;

pub static NOTE_LINE_DELIMITER: &str = "||";
pub static NOTE_PRINT_DELIMITER: &str = " --> ";

pub(crate) struct Note {
    pub content: String,
    pub date: String,
}

fn get_date_formatted() -> String {
    format!("{}", get_date())
}

impl Note {
    pub fn new (content: String) -> Note {
        Note { content, date: get_date_formatted() }
    }

    pub fn serialize(&self) -> String {
        format!("{}{}{}", self.date, NOTE_LINE_DELIMITER, self.content)
    }

    pub fn parse(line: String) -> Note {
        let mut splited = line.trim().split(NOTE_LINE_DELIMITER);

        Note {
            date: splited.next().unwrap().to_string(),
            content: splited.next().unwrap().to_string(),
        }
    }

    pub fn get_formated_for_print(&self) -> String {
        format!("{} {} {}", self.date, NOTE_PRINT_DELIMITER, self.content)
    }
    
    // @todo: Provide way to customize format
    pub fn print(&self) {
        println!("{}", self.get_formated_for_print());
    }
}

impl ToString for Note {
    fn to_string(&self) -> String { self.get_formated_for_print() }
}
