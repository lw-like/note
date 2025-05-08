use chrono::{DateTime, Utc};
use chrono_tz::{Europe::Warsaw, Tz};

pub struct DateService;

static DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d";
static DEFAULT_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

impl DateService {
    pub fn now() -> DateTime<Tz> {
        Utc::now().with_timezone(&Warsaw)
    }
    
    pub fn format(fmt: &str) -> String {
        DateService::now().format(fmt).to_string()
    }

    pub fn now_formated() -> String {
        DateService::format(DEFAULT_DATETIME_FORMAT)
    }

    pub fn date_formated() -> String {
        DateService::format(DEFAULT_DATE_FORMAT)
    }
}