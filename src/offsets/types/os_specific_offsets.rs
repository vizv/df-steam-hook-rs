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
