use std::collections::BTreeMap;

use super::platform;

#[derive(Debug, serde::Deserialize)]
pub struct Checksum {
  pub os: String,
  pub platform: String,
  pub checksum: String,
}

pub type Checksums = BTreeMap<String, BTreeMap<u32, String>>;

#[derive(Debug, serde::Deserialize)]
pub struct OsSpecificOffsets {
  name: String,
  windows: String,
  linux: String,
}

impl OsSpecificOffsets {
  pub fn pair(&self) -> (String, String) {
    let name = self.name.to_owned();
    match std::env::consts::OS {
      "windows" => (name, self.windows.to_owned()),
      "linux" => (name, self.linux.to_owned()),
      _ => (name, "".into()),
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct PlatformSpecificOffsets {
  name: String,
  windows_itch: String,
  windows_steam: String,
  linux_itch: String,
  linux_steam: String,
}

impl PlatformSpecificOffsets {
  pub fn pair(&self) -> (String, String) {
    let name = self.name.to_owned();
    match platform::PLATFORM.as_str() {
      "windows-itch" => (name, self.windows_itch.to_owned()),
      "windows-steam" => (name, self.windows_steam.to_owned()),
      "linux-itch" => (name, self.linux_itch.to_owned()),
      "linux-steam" => (name, self.linux_steam.to_owned()),
      _ => (name, "".into()),
    }
  }
}

pub type Offsets = BTreeMap<String, usize>;
pub type ModuleOffsets = BTreeMap<String, (String, usize)>;
