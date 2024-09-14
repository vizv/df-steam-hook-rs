use super::super::super::data;
use super::word;

pub struct WildcardMatch<'a> {
  pub placeholder: &'a str,
  pub original: &'a str,
  pub translated: &'a str,
  pub prefix: &'a str,
  pub suffix: &'a str,
  pub remaining: &'a str,
}

pub fn match_wildcard_table<'a, M>(
  wildcard_table: &'a data::WildcardTable,
  remaining: &'a str,
  mut match_placeholder: M,
) -> Vec<WildcardMatch<'a>>
where
  M: FnMut(&'a str) -> Vec<word::WordMatch<'a>>,
{
  let mut ret: Vec<WildcardMatch<'a>> = Default::default();

  let matches = match_wildcard_table_prefix(wildcard_table, remaining);
  for &WildcardPrefixMatch {
    dict,
    prefix,
    remaining,
  } in matches.iter()
  {
    let matches = match_placeholder(remaining);
    for &word::WordMatch {
      translated,
      original,
      remaining,
    } in matches.iter()
    {
      let matches = word::match_dictionary(dict, remaining);
      for &word::WordMatch {
        translated: placeholder,
        original: suffix,
        remaining,
      } in matches.iter()
      {
        ret.push(WildcardMatch {
          placeholder,
          original,
          translated,
          prefix,
          suffix,
          remaining,
        })
      }
    }
  }

  ret
}

struct WildcardPrefixMatch<'a> {
  dict: &'a data::Dictionary,
  prefix: &'a str,
  remaining: &'a str,
}

fn match_wildcard_table_prefix<'a>(
  wildcard_table: &'a data::WildcardTable,
  remaining: &'a str,
) -> Vec<WildcardPrefixMatch<'a>> {
  let mut ret: Vec<WildcardPrefixMatch<'a>> = Default::default();

  // match empty prefix
  if let Some(dict) = wildcard_table.get("") {
    ret.push(WildcardPrefixMatch {
      dict,
      prefix: "",
      remaining,
    });
  }

  for (i, (pos, _)) in remaining.match_indices(' ').enumerate() {
    let word_count = i + 1;
    if word_count > wildcard_table.max_count {
      break;
    }

    // split by the space, the the current prefix and remaining part
    let (prefix, remaining) = remaining.split_at(pos);
    let remaining = &remaining[1..];

    // lookup wildcard table for prefix, run the closure for each match with translated string
    if let Some(dict) = wildcard_table.get(prefix) {
      ret.push(WildcardPrefixMatch {
        dict,
        prefix,
        remaining,
      });
    }
  }

  ret
}
