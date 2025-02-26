use clock::Clock;
use time::{macros::datetime, OffsetDateTime, PrimitiveDateTime as DateTime};

// mod learning_time_crate;
mod clock;

fn main() {
    println!("1) hello = {}", hello());
    println!("2) reverse_string = {}", reverse_string("stressed"));
    println!("3) after = {}", after(datetime!(2011-04-25 00:00:00)));
    println!("4) clock = {}", Clock::new(2, 20).add_minutes(-3000));

    // let a = u8::MAX;
    // println!("{:?}", a.wrapping_add(1));

}

fn after(start: DateTime) -> DateTime {
    // another way to initialize the value without macros
    /* let primitive_date_time = Date::from_calendar_date(2011, time::Month::January, 1)
        .unwrap()
        .with_hms(0, 0, 0)
        .unwrap();
    */

    let after_timestamp = start
        .assume_utc()
        .unix_timestamp()
        .checked_add(gigasecond())
        .unwrap();
    let after_offset = OffsetDateTime::from_unix_timestamp(after_timestamp).unwrap();
    DateTime::new(after_offset.date(), after_offset.time())
}

fn gigasecond() -> i64 {
    10i64.checked_pow(9).unwrap()
}

fn reverse_string(input: &str) -> String {
    let mut vec: Vec<char> = vec![];

    input.chars().rev().for_each(|c| {
        vec.push(c);
    });

    String::from_iter(&vec)
}

fn hello() -> &'static str {
    "Hello, world!"
}
