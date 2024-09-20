use crate::df;

pub fn get_viewscreen() -> Option<String> {
  if let Some(prefix) = df::gview::deref_current_viewscreen(*df::globals::GVIEW) {
    let suffix = "default";
    return Some(format!("{prefix}/{suffix}"));
  }

  None
}
