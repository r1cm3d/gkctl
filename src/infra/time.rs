use crate::types::Time;
use chrono::Utc;

pub struct Chrono {}

impl Time for Chrono {
    fn cur_date_time(&self, format: &str) -> String {
        let now = Utc::now();
        format!("{}", now.format(format))
    }
}