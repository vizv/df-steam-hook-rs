use std::fs::File;

use indexmap::IndexMap;

use super::alignment;

#[static_init::dynamic]
pub static INTERFACES: Interfaces = {
  let mut interfaces = Interfaces::default();

  let file = File::open("./dfint-data/translations/interfaces.csv").unwrap();
  let mut reader = csv::Reader::from_reader(file);
  for result in reader.deserialize() {
    let Interface {
      viewscreen,
      context,
      alignment,
      text,
      text_translation,
    } = result.unwrap();

    if !interfaces.contains_key(&viewscreen) {
      interfaces.insert(viewscreen.clone(), Default::default());
    }
    let contexts = interfaces.get_mut(&viewscreen).unwrap();

    if !contexts.contains_key(&context) {
      contexts.insert(context.clone(), Default::default());
    }
    let dictionary = contexts.get_mut(&context).unwrap();
    dictionary.insert(text, (text_translation, alignment.as_str().into()));
  }

  // println!("{interfaces:#?}");

  interfaces
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
