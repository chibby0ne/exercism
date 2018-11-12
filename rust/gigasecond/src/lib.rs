extern crate chrono;
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGASECOND : i64 = 1_000_000_000;
    let duration = Duration::seconds(GIGASECOND);
    match start.checked_add_signed(duration) {
       Some(x)  => return x,
       None => return start,
    }
}
