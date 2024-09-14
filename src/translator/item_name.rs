use regex::Regex;

use super::{data, matcher};

const C_NOT_OWNER: &str = r"\$";
const C_ON_FIRE: &str = "‼";
const S_WEAR: &str = "[xX]|XX";
const CL_OFF_SITE: &str = r"\(";
const CR_OFF_SITE: &str = r"\)";
const CL_UNCLAIM: &str = r"\{";
const CR_UNCLAIM: &str = r"\}";
const C_QUALITY: &str = "[-+*≡☼]";
const CL_DECOR: &str = "«";
const CR_DECOR: &str = "»";
const CL_MAGIC: &str = "◄";
const CR_MAGIC: &str = "►";

const TOKEN_COUNT: usize = 10;
#[static_init::dynamic]
static DESIGNATION_PREFIX_REGEX: Regex = Regex::new(&format!("^({C_NOT_OWNER})?({C_ON_FIRE})?({S_WEAR})?({CL_OFF_SITE})?({CL_UNCLAIM})?(?:({C_QUALITY})?({CL_DECOR}))?({CL_MAGIC})?({C_QUALITY})?")).unwrap();
#[static_init::dynamic]
static DESIGNATION_SUFFIX_REGEX: Regex = Regex::new(&format!("({C_QUALITY})?({CR_MAGIC})?(?:({CR_DECOR})({C_QUALITY})?)?({CR_UNCLAIM})?({CR_OFF_SITE})?({S_WEAR})?({C_ON_FIRE})?({C_NOT_OWNER})?$")).unwrap();

fn designation_wrapper_matcher<'a>() -> matcher::MatchFn<'a> {
  Box::new(|remaining| {
    let mut remaining = remaining;
    let mut prefix = "";
    let mut suffix = "";

    // match designation prefix and suffix
    if let (Some(prefix_match), Some(suffix_match)) = (
      DESIGNATION_PREFIX_REGEX.find(remaining),
      DESIGNATION_SUFFIX_REGEX.find(remaining),
    ) {
      if !prefix_match.is_empty() && !suffix_match.is_empty() {
        let prefix_caps = DESIGNATION_PREFIX_REGEX.captures(remaining).unwrap();
        let suffix_caps = DESIGNATION_SUFFIX_REGEX.captures(remaining).unwrap();
        let mut prefix_len = 0;
        let mut suffix_len = 0;
        for i in 1..TOKEN_COUNT {
          if let (Some(prefix_token), Some(suffix_token)) = (prefix_caps.get(i), suffix_caps.get(TOKEN_COUNT - i)) {
            let prefix_token = prefix_token.as_str();
            let suffix_token = suffix_token.as_str();
            if match i {
              1 | 2 | 4 | 5 | 7 | 8 => true,
              _ => prefix_token == suffix_token,
            } {
              prefix_len += prefix_token.len();
              suffix_len += suffix_token.len();
            }
          }
        }
        (prefix, remaining) = remaining.split_at(prefix_len);
        (remaining, suffix) = remaining.split_at(remaining.len() - suffix_len);
      }
    }

    let wrapper = vec![prefix, "{}", suffix].concat();
    vec![(remaining, wrapper)]
  })
}

#[static_init::dynamic]
static COUNT_SUFFIX_REGEX: Regex = Regex::new(r" \[\d+\]$").unwrap();

fn item_count_suffix_matcher<'a>() -> matcher::MatchFn<'a> {
  Box::new(|remaining| {
    let mut remaining = remaining;
    let mut suffix = "";

    if let Some(count_match) = COUNT_SUFFIX_REGEX.find(remaining) {
      (remaining, suffix) = remaining.split_at(count_match.start());
    }

    let wrapper = vec!["{}", suffix].concat();
    vec![(remaining, wrapper)]
  })
}

pub fn translate_item_name(string: &String) -> Option<String> {
  let materials_adjectives_matcher = matcher::word_matcher(matcher::DictType::Dictionary(&data::MATERIALS.adjectives));
  let designation_wrapper_matcher = designation_wrapper_matcher();
  let item_count_suffix_matcher = item_count_suffix_matcher();
  let items_wildcard_matcher = matcher::wildcard_matcher(
    &data::ITEMS.wildcard_table,
    move |remaining| materials_adjectives_matcher(remaining),
    |placeholder, translated| placeholder.replace("{}", &translated),
  );
  let remaining = string;
  for (remaining, designation_wrapper) in designation_wrapper_matcher(remaining) {
    for (remaining, item_count_suffix) in item_count_suffix_matcher(remaining) {
      for (remaining, item_translated) in items_wildcard_matcher(remaining) {
        if remaining.is_empty() {
          let translated = item_translated;
          let translated = item_count_suffix.replace("{}", &translated);
          let translated = designation_wrapper.replace("{}", &translated);
          return Some(translated);
        }
      }
    }
  }

  None
}
