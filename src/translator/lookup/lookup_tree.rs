use std::collections::{BTreeMap, HashSet, VecDeque};
use std::{fmt::Debug, iter::once};

#[derive(Debug, Default)]
pub struct LookupTree {
  pub namespace: String,
  pub prefixes: Vec<super::LookupTable>,
  pub main: super::LookupTable,
  pub suffixes: Vec<super::LookupTable>,
  pub children: BTreeMap<String, LookupTree>,
}

impl LookupTree {
  pub fn new(namespace: String) -> Self {
    let main = super::LookupTable::new(namespace.to_owned(), format!("{namespace}:@"));

    Self {
      namespace,
      main,
      ..Self::default()
    }
  }

  pub fn insert(&mut self, kind: String, key: String, value: String) {
    let table = self.get_table_or_create(&kind);
    table.insert(key, value);
  }

  pub fn get_lookup(&self, kind: &str) -> Option<&LookupTree> {
    let mut remaining = kind.trim_start_matches(':').to_owned();
    let mut lookup = self;

    while !remaining.is_empty() {
      let (name, tail) = remaining.split_once(':').unwrap_or((remaining.as_str(), ""));
      if let Some(next) = lookup.children.get(name) {
        lookup = next;
        remaining = tail.to_owned();
      } else {
        return None;
      }
    }

    Some(&lookup)
  }

  #[allow(unused)] // TODO: remove this, as we use top directly?
  pub fn get_table(&self, kind: &str) -> &super::LookupTable {
    let kind = kind.trim_start_matches(':');
    let mut lookup = self;

    for ns in kind.split(':') {
      if ns.starts_with('^') || ns.starts_with('$') {
        let (sym, order) = ns.split_at(1);
        let tables = match sym {
          "^" => &lookup.prefixes,
          "$" => &lookup.suffixes,
          _ => panic!(),
        };

        let order = usize::from_str_radix(order, 10).unwrap();
        return &tables[order];
      }

      lookup = lookup.children.get(ns).unwrap();
    }

    &lookup.main
  }

  pub fn get_table_or_create(&mut self, kind: &str) -> &mut super::LookupTable {
    let mut lookup = self;

    if !kind.is_empty() {
      for ns in kind.split(':') {
        if ns.starts_with('^') || ns.starts_with('$') {
          let (sym, order) = ns.split_at(1);
          let tables = match sym {
            "^" => &mut lookup.prefixes,
            "$" => &mut lookup.suffixes,
            _ => panic!(),
          };

          let order = usize::from_str_radix(order, 10).unwrap();
          if order + 1 > tables.len() {
            tables.resize_with(order + 1, Default::default);
            tables[order].namespace = lookup.namespace.to_owned();
            tables[order].name = format!("{}:{}", lookup.namespace, ns);
          }
          return &mut tables[order];
        }

        lookup = lookup.children.entry(ns.to_owned()).or_insert(Self::new(format!("{}:{ns}", lookup.namespace)));
      }
    }

    &mut lookup.main
  }

  // TODO: add a boolean for early return (return first fully matched segment as vector)
  pub fn lookup(&self, text: &str) -> super::TranslatedSegments {
    let tables: Vec<&super::LookupTable> =
      self.prefixes.iter().chain(once(&self.main)).chain(self.suffixes.iter()).collect();

    let mut ret = HashSet::new();
    let bootstrap = super::TranslatedSegment::from(text);
    let mut pending = VecDeque::new();
    pending.push_back((0, bootstrap));
    // log::debug!(">>> {:?}: lookup {text:?}", self.namespace);

    while let Some((curr_table_index, curr_segment)) = pending.pop_front() {
      let (original, remaining) = curr_segment.split().expect("split segment failed for lookup");

      // log::debug!(
      //   "??? {:?} {}: text: {text:?}, original = {original:?}, remaining = {remaining:?}",
      //   self.namespace,
      //   curr_segment.pos
      // );
      // log::debug!("??? {curr_table_index}: {curr_segment:?}");
      if remaining.is_empty() || curr_table_index >= tables.len() {
        ret.insert(curr_segment.to_owned());
        continue;
      }

      let segments = tables[curr_table_index].lookup(remaining);
      for segment in segments {
        // log::debug!(
        //   "%%% {:?}: {:?} table lookup result = {segment:?}",
        //   self.namespace,
        //   tables[curr_table_index].name
        // );

        // optional table match found!
        if original.is_empty() && segment.translated.is_empty() {
          pending.push_back((curr_table_index + 1, curr_segment.to_owned()));
          continue;
        }

        let next_segment = curr_segment.append(&segment);

        pending.push_back((curr_table_index + 1, next_segment));
      }
    }

    // log::debug!("<<< {:?} lookup {text:?}: {ret:#?}", self.namespace);
    ret.into_iter().collect()
  }

  pub fn get(&self, text: &str) -> Option<String> {
    // TODO: use boolean for early return
    for segment in self.lookup(text) {
      if segment.remaining().is_empty() {
        return Some(segment.translated);
      }
    }

    None
  }

  #[allow(unused)]
  pub fn dump_all(&self, lookup: &str) {
    self.get_lookup(lookup).unwrap().dump(0);
  }

  pub fn dump(&self, level: usize) {
    let sp = "  ".repeat(level);
    println!("{sp}{}:", self.namespace);
    for (i, prefix) in self.prefixes.iter().enumerate() {
      prefix.dump_all(format!("^{i}"), level + 1);
    }

    if self.main.max_count != 0 {
      self.main.dump_all("@".into(), level + 1);
    }

    for (i, suffix) in self.suffixes.iter().enumerate() {
      println!("{sp}${i}({}):", suffix.max_count);
      suffix.dump_all(format!("${i}"), level + 1);
    }

    for (_, child) in &self.children {
      child.dump(level + 1);
    }
  }

  pub fn load_csv(&mut self, file: &str) {
    #[derive(serde::Deserialize)]
    struct LookupEntry {
      table: String,
      text: String, // TODO: change to token
      translation: String,
    }

    crate::utils::load_csv(
      crate::utils::translations_path(file),
      |LookupEntry {
         table,
         text,
         translation,
       }| {
        self.insert(table.to_owned(), text.to_owned(), translation.to_owned());
      },
    );
  }

  pub fn enable(&mut self, lookup: &str) {
    self.insert("".to_owned(), format!("{{:{lookup}}}"), format!("{{:{lookup}}}"));
  }
}
