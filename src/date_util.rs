use chrono::{prelude::*, Duration};

/// return the begin `datetime` of the day in a `datetime` with a specific hour 
/// to determine from which hour one day will begin. 
/// 
/// return `None` if the specific hour >= 24 
// fn get_day_start(start_hour: u32, datetime: NaiveDateTime) -> Option<NaiveDateTime> {
//     if start_hour >= 24 {
//         return None;
//     }
//     Some(get_belong_day(start_hour, datetime).and_hms_opt(start_hour, 0, 0).unwrap())
// }

/// get the date with a specific start hour of a day
pub fn get_belong_day(start_hour: u32, datetime: NaiveDateTime) -> NaiveDate {
    if datetime.hour() >= start_hour { datetime.date() } else { datetime.date() - Duration::try_days(1).unwrap() }
}

/// return a date's begin time and end time (both inclusive) with a specific hour to determine from which hour today will begin. 
/// panic if the specific hour >= 24 
pub fn get_day_range(start_hour: u32, date: NaiveDate) -> (NaiveDateTime, NaiveDateTime) {
    if start_hour >= 24 {
        panic!("start_hour cannot be bigger than 24")
    }
    let start = date.and_hms_opt(start_hour, 0, 0).unwrap();
    let end = start + Duration::try_days(1).unwrap() - Duration::try_seconds(1).unwrap();
    (start, end)
}

/// return today's begin time and end time (both inclusive) with a specific hour to determine from which hour today will begin. 
/// panic if the specific hour >= 24 
/// 
/// for example, when at `2023-01-15 08:00:00`, `today_range(6) = Some('2023-01-15 06:00:00', '2023-01-16 05:59:59')`, 
/// `today_range(9) = Some('2023-01-14 09:00:00', '2023-01-15 08:59:59')`, 
pub fn today_range(start_hour: u32) -> (NaiveDateTime, NaiveDateTime) {
    last_n_day_range(start_hour, 1)
}

pub fn last_n_day_range(start_hour: u32, days: u32) -> (NaiveDateTime, NaiveDateTime) {
    let today = get_belong_day(start_hour, Local::now().naive_local());
    let last_n_day = today + Duration::try_days(-(days as i64) + 1).unwrap();
    let (start, _) = get_day_range(start_hour, last_n_day);
    let (_, end) = get_day_range(start_hour, today);
    (start, end)
}

pub fn day_range_iter(start_hour: u32, start_date: NaiveDate, end_date: NaiveDate) -> impl Iterator<Item=(NaiveDate, NaiveDateTime, NaiveDateTime)> {
    start_date.iter_days().take_while(move |day| *day <= end_date).map(move |day| {
        let (start, end) = get_day_range(start_hour, day);
        (day, start, end)
    })
}