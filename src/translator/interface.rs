use crate::encodings;

use super::data;

pub fn translate_interface(vs_opt: Option<&str>, ctx_opt: Option<&str>, string: &str) -> Option<(String, i32)> {
  if let Some(vs) = vs_opt {
    let context = ctx_opt.unwrap_or("");
    if let Some(contexts) = data::INTERFACES.get(vs) {
      if let Some(dictonary) = contexts.get(context) {
        if let Some((translation, alignment)) = dictonary.get(string) {
          let string_len = encodings::string_width_in_pixels(string);
          let translation_len = encodings::string_width_in_pixels(translation);
          let diff = string_len as i32 - translation_len as i32;
          let offset = match alignment {
            data::Alignment::LEFT => 0,
            data::Alignment::CENTER => diff / 2,
            data::Alignment::RIGHT => diff,
          };
          return Some((translation.to_owned(), offset));
        }
      }
    }
  }

  None
}
