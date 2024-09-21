use super::data;

pub fn get_context<'a>(vs_opt: Option<&str>, bt: &str) -> Option<&'a str> {
  if let Some(vs) = vs_opt {
    if let Some(contexts) = data::CONTEXTS.get(vs) {
      if let Some(context) = bt.split('/').into_iter().find_map(|offset| contexts.get(offset)) {
        return Some(context);
      }
    }
  }

  None
}
