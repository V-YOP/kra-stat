use std::{collections::HashMap, env, fmt::format, fs, i64::MAX, process};

use anyhow::Result;
use chrono::{Datelike, Duration, Local, NaiveDate};

use date_util::get_day_range;
use kra_history::KraHistory;
use serde_json::json;

use crate::date_util::{day_range_iter, get_belong_day};
use crate::rainbow::{Color, Colorful};
mod kra_stat;
mod kra_history;
mod date_util;
mod rainbow;

const DAY_INTERVAL_HOUR: u32 = 6;

fn print_stat() -> Result<()> {
    let history = KraHistory::init()?;
    let today = get_belong_day(DAY_INTERVAL_HOUR, Local::now().naive_local());

    for (date, start, end) in day_range_iter(DAY_INTERVAL_HOUR, today - Duration::try_days(29).unwrap(), today) {
        let sum = history.between_inclusive(start, end).map(|x| x.duration as f64).sum::<f64>() / 60.0;
        let day_weekday = date.weekday();
        println!("{dayStr} {day_weekday}: {start} ~ {end}: {sum:.0} minutes", dayStr=if date == today {"today".to_owned()} else {format!("{}", date)});
    };
    
    Ok(())
}

fn base_stat(history: &KraHistory) -> Result<String> { 
    let today = get_belong_day(DAY_INTERVAL_HOUR, Local::now().naive_local());
    let year = today.year_ce().1;
    let year_start = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
    let year_end = year_start.with_year(year as i32 + 1).unwrap() - Duration::try_days(1).unwrap();

    let mut res: HashMap<NaiveDate, u32> = HashMap::with_capacity(365);
    let items = history.between_inclusive(get_day_range(DAY_INTERVAL_HOUR, year_start).0, get_day_range(DAY_INTERVAL_HOUR, year_end).1).collect::<Vec<_>>();
    for &item in &items {
        let stat_date = get_belong_day(DAY_INTERVAL_HOUR, item.time);
        res.entry(stat_date).and_modify(|r| { *r += item.duration as u32 }).or_insert(item.duration as u32);
    }
    let max = res.iter().map(|x| *x.1).max().unwrap_or(0) / 60;
    let sum = res.iter().map(|x| *x.1).sum::<u32>() / 60;
    let average = sum / res.len() as u32;
    Ok(format!(r##"appendElem("100%", null, "<h3>max: {max}, sum: {sum}, average: {average}</h3>")"##))
}

fn calendar_heat_map(history: &KraHistory) -> Result<String> {
    let today = get_belong_day(DAY_INTERVAL_HOUR, Local::now().naive_local());
    let year = today.year_ce().1;
    let year_start = NaiveDate::from_ymd_opt(year as i32, 1, 1).unwrap();
    let year_end = year_start.with_year(year as i32 + 1).unwrap() - Duration::try_days(1).unwrap();

    let mut res: HashMap<NaiveDate, u32> = HashMap::with_capacity(365);
    for date in year_start.iter_days().take_while(|d| *d <= today) {
        res.insert(date, 0);    
    }
    let items = history.between_inclusive(get_day_range(DAY_INTERVAL_HOUR, year_start).0, get_day_range(DAY_INTERVAL_HOUR, year_end).1);
    for item in items {
        let stat_date = get_belong_day(DAY_INTERVAL_HOUR, item.time);
        res.entry(stat_date).and_modify(|r| { *r += item.duration as u32 }).or_insert(item.duration as u32);
    }
    let res = year_start.iter_days()
        .take_while(|date| date.year_ce().1 == year)
        .filter_map(|date| {
            res.get(&date).map(|v| (date.format("%Y-%m-%d").to_string(), v / 60))
        })
        .collect::<Vec<_>>();
    let option = json!({
        "title": {
            "top": 30,
            "left": "center",
            "text": format!("{} 年每日绘画时间（分）", year)
        },
        "tooltip": { "formatter": "{c}" },
        "visualMap": {
            "pieces": [
                {"min": 240, "color": "#375093"},
                {"max": 240, "color": "#4E70AF"},
                {"max": 180, "color": "#7091C7"},
                {"max": 120, "color":"#9EBCDB"},
                {"max": 90, "color": "#C8D6E7"},
                {"max": 60, "color": "#E8EDF1"},
                {"max": 30, "color": "#C26B57"},
            ],
            "min": 0,
            "max": res.iter().map(|x| x.1).max(),
            "type": "piecewise",
            "orient": "horizontal",
            "left": "center",
            "top": 65,
            "inRange": { "color": ["#ECF3FF", "#1061ec"] },
        },
        "calendar": {
            "top": 120,
            "left": 30,
            "right": 30,
            // "cellSize": ["auto", "auto"],
            "range": year,
            "itemStyle": {
                "borderWidth": 5,
                "borderJoin": "round",
                "borderColor": "#fafafa",
                "color": "#cacaca",
            },
            "cellSize": [24, 24],
            "splitLine": true,
            "dayLabel": {
                "firstDay": 1,
                "nameMap": "ZH",
            },
            "yearLabel": {
                "show": false
            },
            "monthLabel": {
                "nameMap": ["一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"]
            }
        },
        "series": {
            "type": "heatmap",
            "coordinateSystem": "calendar",
            "data": res
        }
    });
    Ok(format!("appendChart('100%', '300px', {})", option))
}

fn generate_html() -> Result<()> {
    let template = include_str!("./asset/template.md.hbs");
    let echart_min_js = include_str!("./asset/echarts.min.js");
    let index_css = include_str!("./asset/styling.css");

    let cal_heatmap_css = include_str!("./asset/cal-heatmap.css");
    let d3_v7_min_js = include_str!("./asset/d3.v7.min.js");
    let cal_heatmap_min_js = include_str!("./asset/cal-heatmap.min.js");
    let handlebars = handlebars::Handlebars::new();
    let history = KraHistory::init()?;
    let res = handlebars.render_template(template, &serde_json::json!({
        "title": "my title",
        "echarts_min_js": echart_min_js,
        "index_css": index_css,
        "cal_heatmap_css": cal_heatmap_css,
        "d3_v7_min_js": d3_v7_min_js,
        "cal_heatmap_min_js": cal_heatmap_min_js,
        "elems": [
            base_stat(&history)?,
            calendar_heat_map(&history)?,
        ]
    }))?;
    let target_md_path = env::temp_dir().join("kra-stat.md");
    fs::write(&target_md_path, res)?;
    let target_html_path = target_md_path.with_file_name("kra-stat.html");
    process::Command::new("pandoc")
        .arg("-o").arg(&target_html_path)
        .arg(target_md_path)
        .args(&[ "--css", "tufte.css"])
        .spawn()?.wait()?;
    process::Command::new("cmd")
        .args(&["/C", "start"])
        .arg(&target_html_path)
        .spawn()?.wait()?;
    Ok(())
}


fn f() -> Result<()> {
    let history = KraHistory::init()?;
    
    Ok(())
}


fn main() -> Result<()> {
    
    // print_stat()?;
    // generate_html()?;
    println!("{}", "hello".fg(Color::RGB(255, 255, 255)).bg(Color::RGB(0, 0, 0)));
    println!("{}", "\u{3000}".underline());
    Ok(())
}

