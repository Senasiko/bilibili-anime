use crate::anime::Anime;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DayResult {
  date_ts: i64,
  day_of_week: i8,
  is_today: i8,
  pub date: String,
  pub seasons: Vec<Anime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BiliResponse {
  code: i8,
  message: String,
  result: Vec<DayResult>,
}

pub async fn request_bangumi() -> Result<Vec<DayResult>, Box<dyn std::error::Error>> {
  const API_URL: &str = "https://bangumi.bilibili.com/web_api/timeline_global";
  let resp_text = reqwest::get(API_URL).await?.text().await?;
  let resp: BiliResponse = serde_json::from_str(&resp_text).expect("parse response error");
  Ok(resp.result)
}

pub fn get_animes_by_date<'a>(days: &'a Vec<DayResult>, date: &i64) -> Option<Vec<&'a Anime>> {
  let today_result = days.iter().find(|day| day.date_ts == *date);
  if let Some(today) = today_result {
    Some(today.seasons.iter().map(|anime| anime).collect())
  } else {
    None
  }
}
