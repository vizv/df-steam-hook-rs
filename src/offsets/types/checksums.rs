use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize)]
pub struct Checksum {
  pub os: String,
  pub platform: String,
  pub checksum: String,
}

pub type Checksums = BTreeMap<String, BTreeMap<u32, String>>;
