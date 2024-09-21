use anyhow::{anyhow, Result};
use std::io::Read;

#[derive(Debug)]
pub struct Settings {
  pub log_level: log::LevelFilter,
  pub log_file: String,
  pub font_file: String,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      log_level: log::LevelFilter::Debug,
      log_file: "./dfint-data/dfint-log.log".into(),
      font_file: "./dfint-data/fonts/NotoSansMonoCJKsc-Bold.otf".into(),
    }
  }
}

impl Settings {
  pub fn load() -> Result<Self> {
    let mut settings = Self::default();

    let mut file = std::fs::File::open("./dfint-data/config.txt")?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;

    for cap in regex::bytes::Regex::new(r"\[([^\]:]+):([^\]]+)\]")?.captures_iter(&contents) {
      let key = String::from_utf8_lossy(&cap[1]).into_owned();
      let value = String::from_utf8_lossy(&cap[2]).into_owned();
      match key.as_str() {
        "LOG_LEVEL" => match value.as_str() {
          "Trace" => settings.log_level = log::LevelFilter::Trace,
          "Debug" => settings.log_level = log::LevelFilter::Debug,
          "Info" => settings.log_level = log::LevelFilter::Info,
          "Warn" => settings.log_level = log::LevelFilter::Warn,
          "Error" => settings.log_level = log::LevelFilter::Error,
          "Off" => settings.log_level = log::LevelFilter::Off,
          _ => return Err(anyhow!("忽略无效的日志级别：{value:?}")),
        },
        "LOG_FILE" => settings.log_file = value,
        "FONT_FILE" => settings.font_file = value,
        _ => return Err(anyhow!("忽略无效的配置项：{key:?}")),
      }
    }

    Ok(settings)
  }
}
