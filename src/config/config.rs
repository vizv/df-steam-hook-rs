use anyhow::Result;

use crate::utils;

use super::{checksum, offsets, settings};

#[static_init::dynamic]
pub static CONFIG: Config = Config::new();

#[derive(Debug, Default)]
pub struct Config {
  pub offsets: offsets::Offsets,
  pub settings: settings::Settings,
  pub checksum: u32,
  pub version: &'static str,
}

impl Config {
  pub fn new() -> Self {
    match Self::load() {
      Ok(config) => config,
      Err(message) => {
        utils::message_box(
          "dfint 错误",
          message.to_string().as_str(),
          utils::MessageIconType::Error,
        );
        Self::default()
      }
    }
  }

  pub fn load() -> Result<Self> {
    let offsets = offsets::OFFSETS;
    let settings = settings::Settings::load()?;
    let checksum = checksum::CHECKSUM.verify()?;
    let version = match option_env!("HOOK_VERSION") {
      Some(version) => version,
      None => "内部版本",
    };

    Ok(Self {
      offsets,
      checksum,
      settings,
      version,
    })
  }
}
