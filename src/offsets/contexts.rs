use crate::utils;

use super::types;

#[static_init::dynamic]
pub static CONTEXTS: types::Contexts = {
  let mut ret = types::Contexts::default();
  utils::load_csv(utils::offsets_path("contexts.csv"), |context: types::Context| {
    let (view, location, offset) = context.tuple();
    if offset.is_empty() {
      return;
    }

    let contexts = ret.entry(view).or_default();
    contexts.insert(offset, location);
  });
  ret
};
