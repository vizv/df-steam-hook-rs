use std::{collections::HashMap, fs::File};

#[derive(Debug, serde::Deserialize)]
struct HelpDocument {
  title: String,
  title_translation: String,
  document: String,
  document_translation: String,
}

#[derive(Debug, serde::Deserialize)]
struct HelpText {
  text: String,
  text_translation: String,
}

#[static_init::dynamic]
pub static HELP: HashMap<String, String> = {
  let mut help: HashMap<String, String> = Default::default();

  let file = File::open("./dfint-data/translations/help-documents.csv").unwrap();
  let mut reader = csv::Reader::from_reader(file);
  for result in reader.deserialize() {
    let HelpDocument {
      title,
      title_translation,
      document,
      document_translation,
    } = result.unwrap();

    if !title_translation.is_empty() {
      help.insert(title, title_translation);
    }
    if !document_translation.is_empty() {
      help.insert(document, document_translation);
    }
  }

  let file = File::open("./dfint-data/translations/help-texts.csv").unwrap();
  let mut reader = csv::Reader::from_reader(file);
  for result in reader.deserialize() {
    let HelpText { text, text_translation } = result.unwrap();

    if !text_translation.is_empty() {
      help.insert(text, text_translation);
    }
  }

  help
};
