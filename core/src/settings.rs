/* ~~/core/src/settings.rs */

// third-party crates
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
  pub font_family: String,
  pub background_color: String,
  pub text_color: String,
  pub middle_letter_color: String,
  pub font_size: String,
  pub full_screen: bool,
  pub width: String,
  pub height: String,
  pub speed_increment: i32,
  pub initial_speed: i32,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      font_family: "monospace".to_string(),
      background_color: "hsl(0, 0%, 15%)".to_string(),
      text_color: "hsl(0, 0%, 90%)".to_string(),
      middle_letter_color: "hsl(25, 50%, 50%)".to_string(),
      font_size: "30px".to_string(),
      full_screen: false,
      width: "90%".to_string(),
      height: "auto".to_string(),
      speed_increment: 30,
      initial_speed: 400,
    }
  }
}

pub async fn load_settings() -> Settings {
  // For now, just return defaults
  // In a full implementation, we'd use chrome.storage.sync.get
  Settings::default()
}

pub async fn save_settings(_settings: &Settings) {
  // For now, do nothing
  // In a full implementation, we'd use chrome.storage.sync.set
}
