use regex::Regex;

use super::super::data;
use super::common;

#[static_init::dynamic]
static COUNT_SUFFIX_REGEX: Regex = Regex::new(r" \[\d+\]$").unwrap();

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

#[derive(Debug, Default)]
struct ItemName<'a> {
  prefix: &'a str,
  suffix: &'a str,
  count: &'a str,
  name: &'a str,
}

impl<'a> ItemName<'a> {
  fn new(string: &'a str) -> Self {
    let mut ret: Self = Self::default();
    ret.name = string;

    // match designation prefix and suffix
    if let (Some(prefix_match), Some(suffix_match)) = (
      DESIGNATION_PREFIX_REGEX.find(ret.name),
      DESIGNATION_SUFFIX_REGEX.find(ret.name),
    ) {
      if !prefix_match.is_empty() && !suffix_match.is_empty() {
        let prefix_caps = DESIGNATION_PREFIX_REGEX.captures(ret.name).unwrap();
        let suffix_caps = DESIGNATION_SUFFIX_REGEX.captures(ret.name).unwrap();
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
        (ret.prefix, ret.name) = ret.name.split_at(prefix_len);
        (ret.name, ret.suffix) = ret.name.split_at(string.len() - suffix_len - prefix_len);
      }
    }

    // match count suffix
    if let Some(count_match) = COUNT_SUFFIX_REGEX.find(ret.name) {
      (ret.name, ret.count) = ret.name.split_at(count_match.start());
    }

    ret
  }

  fn wrap(&self, name: &str) -> String {
    vec![self.prefix, &name, self.count, self.suffix].concat()
  }
}

pub fn match_item_name(string: &String) -> Option<String> {
  let item = ItemName::new(string);
  let matches = common::match_wildcard_table(
    &data::ITEMS.wildcard_table,
    item.name,
    |remaining| common::deprecated_match_dictionary(&data::MATERIALS.adjectives, remaining),
    |placeholder, translated, prefix, original, suffix| {
      let mut translated = placeholder.replace("{}", &translated);
      if let Some(&use_noun_for_adj) =
        data::ITEMS.should_use_noun_for_adj.get(&(prefix.to_string(), suffix.to_string()))
      {
        if use_noun_for_adj {
          if let Some(translated_noun) = data::MATERIALS.nouns.get(original) {
            translated = placeholder.replace("{}", translated_noun);
          }
        }
      }

      translated
    },
  );
  for common::WordMatch {
    translated, remaining, ..
  } in matches.into_iter()
  {
    if remaining.is_empty() {
      return Some(item.wrap(&translated));
    }
  }

  None
}
