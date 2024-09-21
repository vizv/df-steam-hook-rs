use std::fs::File;

use indexmap::IndexMap;

#[static_init::dynamic]
pub static CONTEXTS: Contexts = {
  let mut contexts = Contexts::default();

  let file = File::open("./dfint-data/translations/contexts.csv").unwrap();
  let mut reader = csv::Reader::from_reader(file);
  for result in reader.deserialize() {
    let Context {
      viewscreen,
      context,
      windows_itch,
      windows_steam,
      linux_itch,
      // linux_steam, // TODO: add Linux Steam version support
    } = result.unwrap();

    let value = if cfg!(all(target_os = "linux", not(feature = "steam"))) {
      linux_itch
    } else if cfg!(all(target_os = "windows", not(feature = "steam"))) {
      windows_itch
    } else if cfg!(all(target_os = "windows", feature = "steam")) {
      windows_steam
    } else {
      String::new()
    };

    if !contexts.contains_key(&viewscreen) {
      contexts.insert(viewscreen.clone(), Default::default());
    }
    let contexts = contexts.get_mut(&viewscreen).unwrap();
    contexts.insert(value, context);
  }

  // println!("{contexts:#?}");

  contexts
};

#[derive(Debug, serde::Deserialize)]
struct Context {
  viewscreen: String,
  context: String,
  windows_itch: String,
  windows_steam: String,
  linux_itch: String,
  // linux_steam: String, // TODO: add Linux Steam version support
}

pub type Contexts = IndexMap<String, IndexMap<String, String>>;
