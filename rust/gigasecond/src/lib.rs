extern crate time;
use time::PrimitiveDateTime as DateTime;
use time::Duration;

pub fn after(time: DateTime) -> DateTime {
    time + Duration::seconds(1_000_000_000)
}
