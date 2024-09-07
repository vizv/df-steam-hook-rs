use super::super::data;

pub fn match_token<'a, F>(dicts: Vec<&'a data::Dictionary>, remaining: &'a str, mut f: F)
where
  F: FnMut(&'a str, &'a str),
{
  let mut dicts = dicts;
  for (i, (pos, _)) in remaining.match_indices(' ').enumerate() {
    let count = i + 1;
    dicts = dicts.into_iter().filter(|&dict| count <= dict.max_count).collect();
    if dicts.is_empty() {
      break;
    }

    let (prefix, remaining) = remaining.split_at(pos);
    let remaining = &remaining[1..];
    let prefix_key = &prefix.to_lowercase();
    for &dict in dicts.iter() {
      if let Some(translated) = dict.get(prefix_key) {
        f(translated.as_str(), remaining);
      }
    }
  }
  let remaining_key = &remaining.to_lowercase();
  for &dict in dicts.iter() {
    if let Some(translated) = dict.get(remaining_key) {
      f(translated.as_str(), "");
    }
  }
}
