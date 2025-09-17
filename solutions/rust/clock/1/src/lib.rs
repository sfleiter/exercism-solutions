use std::fmt::{self, Display};
use time::{Duration, Time};
//use time::{PrimitiveDateTime as DateTime, datetime};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Clock {
    time: Time,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time =
            Time::MIDNIGHT + Duration::hours(hours as i64) + Duration::minutes(minutes as i64);
        Self { time }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = self.time + Duration::minutes(minutes as i64);
        Self { time }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.time.hour(), self.time.minute())
    }
}
