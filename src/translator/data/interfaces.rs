use indexmap::IndexMap;

use crate::utils;

use super::alignment;

#[static_init::dynamic]
pub static INTERFACES: Interfaces = {
  let mut ret = Interfaces::default();
  utils::load_csv(
    utils::translations_path("interfaces.csv"),
    |Interface {
       viewscreen,
       context,
       alignment,
       text,
       text_translation,
     }| {
      if !ret.contains_key(&viewscreen) {
        ret.insert(viewscreen.clone(), Default::default());
      }
      let contexts = ret.get_mut(&viewscreen).unwrap();

      if !contexts.contains_key(&context) {
        contexts.insert(context.clone(), Default::default());
      }
      let dictionary = contexts.get_mut(&context).unwrap();
      dictionary.insert(text, (text_translation, alignment.as_str().into()));
    },
  );
  ret
};

#[derive(Debug, serde::Deserialize)]
struct Interface {
  viewscreen: String,
  context: String,
  alignment: String,
  text: String,
  text_translation: String,
}

pub type Interfaces = IndexMap<String, IndexMap<String, IndexMap<String, (String, alignment::Alignment)>>>;
