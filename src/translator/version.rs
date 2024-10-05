use crate::{config, offsets};

pub const VERSION: &str = "50.14";

pub fn translate_version(vs_opt: Option<&str>, string: &str) -> Option<String> {
  if let Some(vs) = vs_opt {
    if vs == "title" && string == VERSION {
      return Some(format!(
        "{string} + dfint-rust-cjk/{}-{}",
        *offsets::PLATFORM,
        config::CONFIG.version
      ));
    }
  }

  None
}
