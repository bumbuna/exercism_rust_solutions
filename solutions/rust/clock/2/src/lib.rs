use std::fmt;

const MINUTES_IN_A_DAY: i32 = 1_440;
const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_AN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    mins_past_midnight: i32, // minutes past midnight
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let h = h % HOURS_IN_A_DAY;
        let x = MINUTES_IN_A_DAY + h * MINUTES_IN_AN_HOUR;
        let m = m % MINUTES_IN_A_DAY;
        let x = x + m + MINUTES_IN_A_DAY;
        let x = x % MINUTES_IN_A_DAY;
        Clock {
            mins_past_midnight: x,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = minutes % MINUTES_IN_A_DAY;
        let m = m + self.mins_past_midnight;
        Clock::new(0, m)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:02}:{:02}",
            self.mins_past_midnight / MINUTES_IN_AN_HOUR,
            self.mins_past_midnight % MINUTES_IN_AN_HOUR
        )
    }
}
