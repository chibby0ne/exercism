use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: u32,
    minutes: u32,
}

fn handle_input(hours: i32, minutes: i32) -> (u32, u32) {
    let mut clock_minutes: i32 = minutes % 60;
    if clock_minutes < 0 {
        clock_minutes += 60;
    }
    let mut extra_hours: i32 = minutes / 60;
    if minutes < 0 && minutes % 60 != 0 {
        extra_hours += -1;
    }
    let mut clock_hours: i32 = (hours + extra_hours) % 24;
    if clock_hours < 0 {
        clock_hours += 24;
    }
    (clock_hours as u32, clock_minutes as u32)
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (clock_hours, clock_minutes) = handle_input(hours, minutes);
        Clock {
            hours: clock_hours,
            minutes: clock_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours as i32, self.minutes as i32 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
