use anyhow::Result;

use crate::utils;

use super::settings;

#[static_init::dynamic]
pub static CONFIG: Config = Config::new();

pub struct Config {
  pub settings: settings::Settings,
  pub version: &'static str,
}

impl Config {
  pub fn new() -> Self {
    match Self::load() {
      Ok(config) => config,
      Err(message) => {
        let message = format!("加载配置文件失败：{message}");
        utils::show_error_dialog(&message);
        panic!("{}", message);
      }
    }
  }

  pub fn load() -> Result<Self> {
    let settings = settings::Settings::load()?;
    let version = match option_env!("HOOK_VERSION") {
      Some(version) => version,
      None => "内部版本",
    };

    Ok(Self { settings, version })
  }
}
