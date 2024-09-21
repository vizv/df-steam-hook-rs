use super::super::platform;

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
