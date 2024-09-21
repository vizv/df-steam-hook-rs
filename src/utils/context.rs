use crate::df;

pub fn get_view() -> Option<String> {
  df::gview::get_current_viewscreen_name(*df::globals::GVIEW).map(|s| s.to_owned())
}
