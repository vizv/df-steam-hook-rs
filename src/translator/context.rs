use crate::offsets;

pub fn get_context_location<'a>(view_opt: Option<&str>, bt: &str) -> Option<&'a str> {
  if let Some(view) = view_opt {
    if let Some(contexts) = offsets::CONTEXTS.get(view) {
      if let Some(location) = bt.split('/').into_iter().find_map(|offset| contexts.get(offset)) {
        return Some(location);
      }
    }
  }

  None
}
