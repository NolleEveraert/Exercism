use time::Duration;
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let offset: i64 = 1_000_000_000;
    return start + Duration::seconds(offset);
}
