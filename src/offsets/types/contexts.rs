use std::collections::BTreeMap;

use super::super::platform;

#[derive(Debug, serde::Deserialize)]
pub struct Context {
  view: String,
  location: String,
  windows_itch: String,
  windows_steam: String,
  linux_itch: String,
  linux_steam: String,
}

impl Context {
  pub fn tuple(&self) -> (String, String, String) {
    let view = self.view.to_owned();
    let location = self.location.to_owned();
    match platform::PLATFORM.as_str() {
      "windows-itch" => (view, location, self.windows_itch.to_owned()),
      "windows-steam" => (view, location, self.windows_steam.to_owned()),
      "linux-itch" => (view, location, self.linux_itch.to_owned()),
      "linux-steam" => (view, location, self.linux_steam.to_owned()),
      _ => (view, location, "".into()),
    }
  }
}

pub type Contexts = BTreeMap<String, BTreeMap<String, String>>;
