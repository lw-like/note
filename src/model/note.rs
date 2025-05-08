use crate::services::date_service::DateService;

pub static NOTE_LINE_DELIMITER: &str = "||";
pub static NOTE_PRINT_DELIMITER: &str = " --> ";

pub(crate) struct Note {
    pub content: String,
    pub date: String,
}

fn get_date_formatted() -> String {
    format!("{}", DateService::now_formated())
}

impl Note {
    pub fn new (content: String) -> Note {
        Note { 
            content: content.clone(), 
            date: get_date_formatted() 
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}{}{}", self.date, NOTE_LINE_DELIMITER, self.content)
    }

    #[allow(dead_code)]
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
}

impl ToString for Note {
    fn to_string(&self) -> String { self.get_formated_for_print() }
}
