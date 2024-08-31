use anyhow::Result;
use std::collections::HashMap;
use std::io::prelude::*;

use crate::config::CONFIG;
use crate::constants::PATH_DICTIONARY;
use crate::utils;

#[static_init::dynamic]
pub static DICTIONARY: Dictionary = Dictionary::new(PATH_DICTIONARY);

pub struct Dictionary {
  map: HashMap<String, String>,
  path: &'static str,
}

impl Dictionary {
  pub fn new(path: &'static str) -> Self {
    Self {
      map: match Dictionary::load(path) {
        Ok(value) => value,
        Err(_) => {
          log::error!("unable to load dictionary {path}");
          utils::message_box(
            "dfint hook error",
            format!("Unable to load dictionary {path}").as_str(),
            utils::MessageIconType::Warning,
          );
          HashMap::<String, String>::new()
        }
      },
      path,
    }
  }

  pub fn get(&self, key: &String) -> Option<&String> {
    self.map.get(key)
  }

  pub fn size(&self) -> usize {
    self.map.len()
  }

  pub fn _data(&self) -> &HashMap<String, String> {
    &self.map
  }

  pub fn _reload(&mut self) -> Result<()> {
    self.map = Self::load(self.path)?;
    Ok(())
  }

  #[allow(unused_must_use)]
  fn load(path: &str) -> Result<HashMap<String, String>> {
    simple_logging::log_to_file(&CONFIG.settings.log_file, utils::log_level(CONFIG.settings.log_level)).unwrap();
    let mut file = std::fs::File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents);
    let mut map = HashMap::<String, String>::new();
    for item in regex::bytes::Regex::new(r#"(?-u)"(.+)","(.+)""#)?.captures_iter(&contents) {
      let k = String::from_utf8_lossy(&item[1]).into_owned();
      let v = String::from_utf8_lossy(&item[2]).into_owned();
      map.insert(k, v);
    }
    Ok(map)
  }
}
