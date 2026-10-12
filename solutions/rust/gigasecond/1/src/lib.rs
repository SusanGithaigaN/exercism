use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    // 1gs = 1,000,000,000 sec
     // determine date and time after gs
    // convert gs into days? 11, 574, 0741 days
    start + Duration::seconds(1_000_000_000)
}
