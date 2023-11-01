use time::PrimitiveDateTime as DateTime;
use std::time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga_second = 1_000_000_000;
    let new_time = start + Duration::from_secs(giga_second);
    new_time

}
