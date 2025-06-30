use std::fmt::{self};

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Convert current time to total minutes and add
        let current_total: i32 = hours * 60 + minutes;

        // Normalize to 24-hour format
        let normalized: i32 = ((current_total % (24 * 60)) + (24 * 60)) % (24 * 60);

        Clock {
            hours: normalized / 60,
            minutes: normalized % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Convert current time to total minutes and add
        let current_total: i32 = self.hours * 60 + self.minutes;
        let new_total: i32 = current_total + minutes;

        // Normalize to 24-hour format
        let normalized: i32 = ((new_total % (24 * 60)) + (24 * 60)) % (24 * 60);

        Clock {
            hours: normalized / 60,
            minutes: normalized % 60,
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.hours == other.hours && self.minutes == other.minutes;
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
