use crate::{config, offsets};

const VERSION: &str = "50.13";

pub fn translate_version(vs_opt: Option<&str>, string: &str) -> Option<String> {
  if let Some(vs) = vs_opt {
    if vs == "title/default" && string == VERSION {
      return Some(format!(
        "{string} + dfint-rust-cjk/{}-{}",
        *offsets::PLATFORM,
        config::CONFIG.version
      ));
    }
  }

  None
}
