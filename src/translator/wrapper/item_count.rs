use regex::Regex;

#[static_init::dynamic]
static COUNT_SUFFIX_REGEX: Regex = Regex::new(r" \[\d+\]$").unwrap();

pub fn unwrap_item_count(remaining: &str) -> super::wrapper::UnwrapResult {
  let mut suffix_len = 0;

  if let Some(count_match) = COUNT_SUFFIX_REGEX.find(remaining) {
    suffix_len = count_match.len();
  }

  super::wrapper::Wrapper::unwrap(remaining, 0, suffix_len)
}
