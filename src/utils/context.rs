use crate::{df, global};

pub fn get_viewscreen() -> Option<String> {
  if let Some(prefix) = df::gview::deref_current_viewscreen(*global::GVIEW) {
    let suffix = "default";
    return Some(format!("{prefix}/{suffix}"));
  }

  None
}
