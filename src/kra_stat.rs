use std::collections::HashMap;

use chrono::{format::Item, prelude::*, Duration};

use crate::{date_util::get_belong_day, DAY_INTERVAL_HOUR};

fn today() -> chrono::NaiveDate {
    get_belong_day(DAY_INTERVAL_HOUR, Local::now().naive_local())
}

fn this_year() -> (chrono::NaiveDate, chrono::NaiveDate) {
    let year = today().year_ce().1;
    let year_start = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
    let year_end = year_start.with_year(year as i32 + 1).unwrap() - Duration::try_days(1).unwrap();
    (year_start, year_end)
}
