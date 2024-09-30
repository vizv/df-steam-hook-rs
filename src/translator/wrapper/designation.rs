use regex::Regex;

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

pub fn unwrap_designation(remaining: &str) -> super::wrapper::UnwrapResult {
  let mut prefix_len = 0;
  let mut suffix_len = 0;

  // match designation prefix and suffix only if we have at least 3 characters
  if remaining.len() > 2 {
    if let (Some(prefix_match), Some(suffix_match)) = (
      DESIGNATION_PREFIX_REGEX.find(remaining),
      DESIGNATION_SUFFIX_REGEX.find(remaining),
    ) {
      if !prefix_match.is_empty() && !suffix_match.is_empty() {
        let prefix_caps = DESIGNATION_PREFIX_REGEX.captures(remaining).unwrap();
        let suffix_caps = DESIGNATION_SUFFIX_REGEX.captures(remaining).unwrap();
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
      }
    }
  }

  super::wrapper::Wrapper::unwrap(remaining, prefix_len, suffix_len)
}
