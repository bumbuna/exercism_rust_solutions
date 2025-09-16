const MINUTES_IN_A_DAY: i32 = 1_440;
const HOURS_IN_A_DAY: i32 = 24;
const MINUTES_IN_AN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes_in_day: i32, // minutes past midnight
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        let h = h % HOURS_IN_A_DAY;
        let x = MINUTES_IN_A_DAY + h * MINUTES_IN_AN_HOUR;
        let m = m % MINUTES_IN_A_DAY;
        let x = x + m + MINUTES_IN_A_DAY;
        let x = x % MINUTES_IN_A_DAY;
        Clock { minutes_in_day: x }
    }
}

impl Clock {
    pub fn to_string(&self) -> String {
        let h = self.minutes_in_day / MINUTES_IN_AN_HOUR;
        let m = self.minutes_in_day % MINUTES_IN_AN_HOUR;

        let h = if h < 10 {
            format!("0{h}")
        } else {
            format!("{h}")
        };
        let m = if m < 10 {
            format!("0{m}")
        } else {
            format!("{m}")
        };
        format!("{h}:{m}")
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = minutes % MINUTES_IN_A_DAY;
        let m = m + self.minutes_in_day;
        Clock::new(0, m)
    }
}
