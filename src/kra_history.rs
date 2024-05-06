use std::{collections::HashMap, fs};
use dirs;
use anyhow::Result;
use chrono::prelude::*;
use serde_json::map::IntoIter;
use thiserror::Error;



#[derive(Debug)]
pub struct KraHistory {
    items: Vec<KraHistoryItem>,
}

impl KraHistory {
    pub fn init() -> Result<Self> {
        read_history_file().and_then(read_kra_history).map(|items| KraHistory {items})
    }
    pub fn between_inclusive(&self, start: NaiveDateTime, end: NaiveDateTime) -> impl Iterator<Item=&KraHistoryItem> {
        self.items.iter().filter(move |item| item.time >= start && item.time <= end)
    }

    pub fn iter(&self) -> impl Iterator<Item=&KraHistoryItem> {
        self.items.iter()
    } 
}

#[derive(Debug, Clone)]
pub struct KraHistoryItem {
    pub time: NaiveDateTime,
    pub file_path: Option<String>,
    pub file_id: String,
    pub duration: u32
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct KraHistoryError(String);

impl TryFrom<&str> for KraHistoryItem {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self> {
        fn non_empty_string(s: &str) -> Option<String> {
            Some(s).filter(|x| !x.is_empty()).map(str::to_owned)
        }
        let value = value.trim();
        let &[time, file_path, file_id, duration] = value.split("##").collect::<Vec<_>>().as_slice() else {
            return Err(KraHistoryError(format!("'{}' is illegal", value)).into())
        };
        let time = NaiveDateTime::parse_from_str(time, "%Y-%m-%d %H:%M:%S")?;
        let file_path = non_empty_string(file_path);
        let file_id = non_empty_string(file_id).ok_or(KraHistoryError(format!("'{}' have empty file id", value)))?;
        let duration = non_empty_string(duration)
            .ok_or(KraHistoryError(format!("'{}' have empty duration", value)))?
            .parse::<u32>()?;
        Ok(KraHistoryItem { time, file_path, file_id, duration })
    }
}

fn read_history_file() -> Result<String> {
    let history_path = dirs::home_dir().unwrap().join(".kra_history").join("history");
    fs::metadata(&history_path).map_err(|_| KraHistoryError(format!("~/.kra_history/history doesn't exist or has no permission")))?;
    Ok(fs::read_to_string(&history_path)?)
}

fn read_kra_history(content: String) -> Result<Vec<KraHistoryItem>> {
    let mut res: Vec<KraHistoryItem> = vec![];
    let mut latest_name_by_id: HashMap<String, String> = HashMap::new();
    for ele in content.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).rev() {
        let mut ele = KraHistoryItem::try_from(ele)?;
        if let Some(file_path) = &ele.file_path {
            latest_name_by_id.insert(ele.file_id.clone(), file_path.clone());
        }
        if ele.file_path.is_none() && latest_name_by_id.contains_key(&ele.file_id) {
            ele.file_path = Some(latest_name_by_id.get(&ele.file_id).unwrap().clone());
        }
        res.push(ele);
    }
    res.reverse();
    Ok(res)
}

#[cfg(test)]
mod test {
    use crate::{date_util::today_range, kra_history::read_kra_history};

    use super::{read_history_file, KraHistoryItem};

    #[test]
    fn test() {
        let history_item = "2024-03-07 22:50:48##D:/DESKTOP/TUTOR/预科/第1周/第1天/2.复制组合线条.kra##d18810fd-f9de-4ea6-ae42-587ff2d7507a##180";
        dbg!(KraHistoryItem::try_from(history_item).expect("!!"));
        
        let history_item = "2024-03-07 22:50:48####d18810fd-f9de-4ea6-ae42-587ff2d7507a##180";
        dbg!(KraHistoryItem::try_from(history_item).expect("!!"));

        let history_item = "
        2024-03-07 22:48:48####d18810fd-f9de-4ea6-a142-587ff2d7507a##180
        2024-03-07 22:49:48##h1ello.kra##d18810fd-f9de-4ea6-ae42-587ff2d7507a##180
        2024-03-07 22:50:48####d18810fd-f9de-4ea6-ae42-587ff2d7507a##180
        2024-03-07 22:51:48##hello.kra##d18810fd-f9de-4ea6-ae42-587ff2d7507a##180
        ".to_owned();
        dbg!(read_kra_history(history_item).unwrap());
    }

    #[test]
    fn read_history_file_test() {
        let res = read_history_file().and_then(read_kra_history);
        assert!(res.is_ok());
        let res = res.unwrap();
        dbg!(res.len());
        dbg!(res.iter().map(|x| x.duration as i32).sum::<i32>() / 3600);
    }

    #[test]
    fn today_range_test() {
        
        dbg!(today_range(6));
        dbg!(today_range(18));
    }
}