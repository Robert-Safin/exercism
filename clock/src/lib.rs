use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: String,
    minutes: String,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        if hours == 24 && minutes == 0 {
            return Self {
                hours: String::from("00"),
                minutes: String::from("00"),
            };
        }

        if minutes == 60 {
            minutes = 0;
            hours = hours + 1
        }

        if minutes > 60 {
            let multiple = minutes / 60;
            hours = hours + multiple;
            minutes = minutes - (multiple * 60)
        }

        if hours > 24 {
            let multiple = hours / 24;
            hours = hours - (multiple * 24)
        }

        if hours < 0 {
          hours = 24 + hours
        }

        if hours < -24 {
          print!("asdfghjkl;kjhgfdsdfghjkljhgfdsdfghj");
          let multiple = hours / 24;
          hours = (multiple * 24) + hours
        }



        let fmt_hours = if hours < 10 {
            format!("0{hours}")
        } else {
            hours.to_string()
        };

        let fmt_minutes = if minutes < 10 {
            format!("0{minutes}")
        } else {
            minutes.to_string()
        };

        let new_clock = Self {
            hours: fmt_hours,
            minutes: fmt_minutes,
        };
        return new_clock;
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        todo!("Add {minutes} minutes to existing Clock time");
    }
}
