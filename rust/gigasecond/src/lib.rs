use chrono::prelude::*;
use std::time::Duration;
fn test(){
    let dt = Local::now();
    // property accessors
    assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
    assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
    assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
    assert_eq!(dt.weekday(), Weekday::Fri);
    assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sat=7
    assert_eq!(dt.ordinal(), 332); // the day of year
    assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

// time zone accessor and manipulation
    assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
    assert_eq!(dt.timezone(), FixedOffset::east(9 * 3600));
    assert_eq!(dt.with_timezone(&Utc), Utc.ymd(2014, 11, 28).and_hms_nano(12, 45, 59, 324310806));

// a sample of property manipulations (validates dynamically)
    assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
    assert_eq!(dt.with_day(32), None);
    assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

// arithmetic operations
    let dt1 = Utc.ymd(2014, 11, 14).and_hms(8, 9, 10);
    let dt2 = Utc.ymd(2014, 11, 14).and_hms(10, 9, 8);
    assert_eq!(dt1.signed_duration_since(dt2), Duration::seconds(-2 * 3600 + 2));
    assert_eq!(dt2.signed_duration_since(dt1), Duration::seconds(2 * 3600 - 2));
    assert_eq!(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + Duration::seconds(1_000_000_000),
               Utc.ymd(2001, 9, 9).and_hms(1, 46, 40));
    assert_eq!(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) - Duration::seconds(1_000_000_000),
               Utc.ymd(1938, 4, 24).and_hms(22, 13, 20));
}
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::new(10u64.pow(9), 0)
}
