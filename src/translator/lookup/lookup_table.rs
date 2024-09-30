use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::fmt::Debug;
use std::rc::Rc;

use regex::Regex;

#[derive(Debug, Default)]
pub struct LookupResult {
  pub partial: BTreeSet<String>,
  pub lookups: BTreeSet<String>,
  pub matched: Option<String>,
}

#[derive(Debug, Default)]
pub struct LookupTable {
  pub name: String,
  pub namespace: String,
  pub table: BTreeMap<String, LookupResult>,
  pub max_count: usize,
}

#[static_init::dynamic]
static PLACEHOLDER_REGEX: Regex = Regex::new(r"\{[^\}]*\}").unwrap();

impl LookupTable {
  pub fn new(namespace: String, name: String) -> Self {
    Self {
      name,
      namespace,
      ..Self::default()
    }
  }

  fn unwrap_placeholder<'a>(token: &'a str) -> Option<&'a str> {
    if token.starts_with('{') && token.ends_with('}') {
      return Some(&token[1..token.len() - 1]);
    }
    None
  }

  fn wrap_placeholder<'a>(placeholder: &'a str) -> String {
    format!("{{{placeholder}}}")
  }

  fn expand_placeholders(&self, input: String) -> String {
    let output = PLACEHOLDER_REGEX.replace_all(&input, |caps: &regex::Captures| {
      let token = caps[0].to_uppercase();
      if let Some(placeholder) = Self::unwrap_placeholder(&token) {
        let mut placeholder = placeholder.to_owned();
        if !placeholder.starts_with(':') {
          placeholder = format!("{}:{placeholder}", self.namespace);
        }
        Self::wrap_placeholder(&placeholder)
      } else {
        token.to_owned()
      }
    });
    output.into_owned()
  }

  pub fn insert(&mut self, key: String, value: String) {
    let key = self.expand_placeholders(key.to_lowercase());
    let value = self.expand_placeholders(value);

    let count = key.split(" ").count();
    if count > self.max_count {
      self.max_count = count;
    }

    self.set_matched(key.to_owned(), value);
    if key.is_empty() {
      return;
    }

    let mut key = key.as_str();
    for (i, ch) in key.char_indices().rev() {
      if ch != ' ' {
        continue;
      }

      let prefix = &key[0..i];
      let suffix = &key[i + 1..];
      key = prefix;

      self.insert_next(key.to_owned(), suffix.to_owned());
    }
    self.insert_next("".to_owned(), key.to_owned());
  }

  fn set_matched(&mut self, key: String, value: String) {
    self.get_or_default(key).matched = Some(value);
  }

  fn insert_next(&mut self, key: String, next: String) {
    if next.starts_with('{') && next.ends_with('}') {
      self.insert_lookups(key, next);
    } else {
      self.insert_partial(key, next);
    }
  }

  fn insert_partial(&mut self, key: String, next: String) {
    self.get_or_default(key).partial.insert(next);
  }

  fn insert_lookups(&mut self, key: String, next: String) {
    self.get_or_default(key).lookups.insert(next);
  }

  fn get_or_default(&mut self, key: String) -> &mut LookupResult {
    self.table.entry(key).or_default()
  }

  // TODO: add a boolean for early return (return first fully matched segment as vector)
  pub fn lookup(&self, text: &str) -> super::TranslatedSegments {
    let mut ret = HashSet::new();
    let bootstrap = super::TranslatedSegment::from(text);
    let mut pending = VecDeque::new();
    pending.push_back((
      bootstrap,
      "".to_owned(),
      Vec::<(&str, Rc<super::TranslatedSegment>)>::new(),
    ));
    // log::debug!(">>>>>> {:?}: table lookup {text:?}", self.name);

    while let Some((curr_segment, curr_prefix, curr_subs)) = pending.pop_front() {
      let (_, remaining) = curr_segment.split().expect("split segment failed for lookup table");

      // log::debug!(
      //   "?????? {:?} {}: text: {text:?}, original = {original:?}, prefix = {curr_prefix:?}, remaining = {remaining:?}",
      //   self.name,
      //   curr_segment.pos
      // );
      if let Some(result) = self.table.get(&curr_prefix) {
        // full match found!
        if let Some(translated) = &result.matched {
          // log::debug!("????????? full match found {translated:?}");

          // fill placeholder
          let mut translated = translated.to_owned();
          for (placeholder, segment) in curr_subs.to_owned() {
            translated = translated.replace(placeholder, &segment.translated);
          }

          // set translated
          let mut matched_segment = curr_segment.to_owned();
          matched_segment.translated = translated;

          // log::debug!(
          //   "!!!!!!!!! {:?} {}: return {matched_segment:?}",
          //   self.name,
          //   matched_segment.pos
          // );
          ret.insert(matched_segment);
        }

        // partial match failed!
        if remaining.is_empty() {
          // log::debug!("????????? partial match failed");
          continue;
        }

        // partial match found!
        let (next_word, next_segment) = curr_segment.next();
        let next_prefix = format!("{curr_prefix} {next_word}").trim_start().to_owned();
        // log::debug!("????????? partial match found: next_segment = {next_segment:?} next_prefix = {next_prefix:?}");
        pending.push_back((next_segment, next_prefix, curr_subs.to_owned()));

        // try to lookup!
        for lookup in &result.lookups {
          // log::debug!("????????? try lookup: {lookup:?}");
          let sub_lookup = super::TOP
            .get_lookup(Self::unwrap_placeholder(lookup).unwrap())
            .expect(&format!("failed to find lookup {lookup:?}"));
          for segment in sub_lookup.lookup(remaining) {
            // lookup match found!
            // log::debug!("{:?}: !!!!!!!!! FIXME: lookup = {lookup}, segment = {segment:?}", self.name);

            // log::debug!("###### curr_segment = {curr_segment:?}, segment = {segment:?}");
            let next_segment = curr_segment.append(&segment);
            // log::debug!("###### next_segment = {next_segment:?}");
            let next_prefix = format!("{curr_prefix} {lookup}").trim_start().to_owned();
            let mut next_subs = curr_subs.to_owned();
            next_subs.push((lookup.as_str(), Rc::new(segment)));
            pending.push_back((next_segment, next_prefix, next_subs));
          }
        }
      }
    }
    // log::debug!("<<<<<< {:?} table lookup {text:?}: {ret:#?}", self.name);
    // log::debug!("%%% ret = {ret:#?}");
    ret.into_iter().collect()
  }

  pub fn dump_all(&self, name: String, level: usize) {
    let sp = "  ".repeat(level);
    if let Some(start) = self.table.get("") {
      println!("{sp}{}:{name}({}): {:?}", self.namespace, self.max_count, start.matched);
      for key in &start.partial {
        self.dump(key, level + 1);
      }
      for key in &start.lookups {
        self.dump(key, level + 1);
      }
    }
  }

  pub fn dump(&self, key: &str, level: usize) {
    let sp = "  ".repeat(level);
    let result = self.table.get(key).unwrap();
    println!("{sp}{key:?}: {:?}", &result.matched);
    for subkey in &result.partial {
      self.dump(&format!("{key} {subkey}"), level + 1);
    }
    for lookup in &result.lookups {
      self.dump(&format!("{key} {lookup}"), level + 1);
    }
  }
}
