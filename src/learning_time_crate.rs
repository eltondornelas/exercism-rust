use time::format_description::well_known::Iso8601;
use time::macros::{date, datetime, format_description, offset, time};
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, UtcOffset};
use time::{Month, Time, Weekday};


fn learning_time_crate() {
    // draft for the after() solution
    let datetime = datetime!(2011-04-25 00:00:00);
    let offset = datetime!(2011-04-25 00:00:00 UTC);
    let timestamp = offset.unix_timestamp();
    let gigasecond = timestamp + 10i64.checked_pow(9).unwrap();

    let after_offset = OffsetDateTime::from_unix_timestamp(gigasecond).unwrap();
    let after = PrimitiveDateTime::new(after_offset.date(), after_offset.time());

    println!("{datetime}, {offset}, {timestamp}, {gigasecond}, {after_offset}, {after}");

    // learning
    let now = OffsetDateTime::now_utc();
    println!("{now}");

    let date = Date::from_iso_week_date(2022, 1, Weekday::Wednesday).unwrap();
    let date2 = Date::from_calendar_date(2022, time::Month::January, 1).unwrap();
    // let date3 = Date::
    let datetime = date.with_hms(13, 0, 55).unwrap();
    let datetime_off = datetime.assume_offset(UtcOffset::from_hms(1, 2, 3).unwrap());
    println!("{date}, {datetime}, {datetime_off}");

    let date = date!(2022 - 01 - 01);
    let datetime = datetime!(2022-01-01 13:00:55);
    let datetime_off = datetime!(2022-01-01 13:00:55 +1:02:03);
    println!("{date}, {datetime}, {datetime_off}");

    let a = datetime!(2022-01-01 10:00:55);
    let b = datetime!(2022-01-01 13:00:00);
    let duration: Duration = b - a;
    println!("{duration}");

    let c = date!(2022 - 01 - 02);
    let d = Date::from_calendar_date(2022, Month::January, 2).unwrap();
    let e = datetime!(2022-01-02 11:12:13.123_456_789);

    let date = date!(2022 - 01 - 02);

    //// PrimitiveDateTime
    let f = datetime!(2022-01-02 11:12:13.123_456_789);
    // A date with 00:00:00 time
    let g = date.midnight();
    // You can also provide a desired time...
    let h = date.with_hms(11, 12, 13).unwrap();
    // or pass an existing `Time`
    let i = date.with_time(Time::from_hms_nano(11, 12, 13, 123_456_789).unwrap());
    // with macros:
    let j = date.with_time(time!(11:12:13.123_456_789));

    println!("{f}, {g}, {h}, {i}, {j}");

    //// OffsetDateTime
    // When we pass an offset at the end to `datetime!`, it will return an
    // `OffsetDateTime` instead of an `PrimitiveDateTime`
    let k = datetime!(2022-01-02 11:12:13 UTC);
    // With a positive offset:
    let l = datetime!(2022-01-02 11:12:13 +1);
    // and a negative offset:
    let m = datetime!(2022-01-02 11:12:13.123_456_789 -2:34:56);

    let dt = datetime!(2022-01-02 11:12:13);

    // With UTC:
    let n = dt.assume_utc();
    // or with another offset:
    let o = dt.assume_offset(UtcOffset::from_hms(1, 2, 3).unwrap());
    // with macros:
    let p = dt.assume_offset(offset!(-11));

    println!("k={k}, \nl={l}, \nm={m}, \nn={n}, \no={o}, \np={p}");

    let q = PrimitiveDateTime::parse("2022-01-02T11:12:13", &Iso8601::DEFAULT).unwrap();
    let my_format = format_description!("h=[hour],m=[minute],s=[second]");
    let r = Time::parse("h=11,m=12,s=13", &my_format).unwrap();

    println!("q={q}, \nr={r}");
}
