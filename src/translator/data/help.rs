use std::collections::HashMap;

use crate::utils;

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
pub static HELP: Helps = {
  let mut ret = Helps::default();
  utils::load_csv(
    utils::translations_path("help-documents.csv"),
    |HelpDocument {
       title,
       title_translation,
       document,
       document_translation,
     }| {
      if !title_translation.is_empty() {
        ret.insert(title, title_translation);
      }
      if !document_translation.is_empty() {
        ret.insert(document, document_translation);
      }
    },
  );
  utils::load_csv(
    utils::translations_path("help-texts.csv"),
    |HelpText { text, text_translation }| {
      if !text_translation.is_empty() {
        ret.insert(text, text_translation);
      }
    },
  );
  ret
};

pub type Helps = HashMap<String, String>;
