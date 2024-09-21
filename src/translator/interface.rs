use crate::encodings;

use super::data;

pub fn translate_interface(view_opt: Option<&str>, location_opt: Option<&str>, string: &str) -> Option<(String, i32)> {
  if let Some(view) = view_opt {
    let location = location_opt.unwrap_or("");
    if let Some(dictionaries) = data::INTERFACES.get(view) {
      if let Some(dictionary) = dictionaries.get(location) {
        if let Some((translation, alignment)) = dictionary.get(string) {
          let string_length = encodings::string_width_in_pixels(string);
          let translation_length = encodings::string_width_in_pixels(translation);
          let length_diff = string_length as i32 - translation_length as i32;
          let horizontal_shift = match alignment {
            data::Alignment::LEFT => 0,
            data::Alignment::CENTER => length_diff / 2,
            data::Alignment::RIGHT => length_diff,
          };
          return Some((translation.to_owned(), horizontal_shift));
        }
      }
    }
  }

  None
}
