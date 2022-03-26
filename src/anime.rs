use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Anime {
  pub title: String,
  cover: String,
  season_id: i64,
  season_status: i8,
  square_cover: String,
  pub_index: Option<String>,
  pub_time: String,
  url: String,
}
impl std::fmt::Display for Anime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "《{}》 | {} | {} | {}", self.title, self.pub_time, if let Some(index) = &self.pub_index { index.to_string() } else { "".to_string() }, self.url)
  }
}

impl std::fmt::Debug for Anime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "《{}》 | {} | {} | {}", self.title, self.pub_time, if let Some(index) = &self.pub_index { index.to_string() } else { "".to_string() }, self.url)
  }
}
