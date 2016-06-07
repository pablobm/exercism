extern crate chrono;
use chrono::DateTime;
use chrono::Duration;
use chrono::UTC;

pub fn after(time: DateTime<UTC>) -> DateTime<UTC> {
    time + Duration::seconds(1_000_000_000)
}
