use std::{collections::HashMap, fs::File};

use regex::Regex;

use super::{dictionary, placeholder, transformer};

#[static_init::dynamic]
pub static PLANTS: Plants = Plants::new();

#[derive(Debug, serde::Deserialize)]
struct PlantBase {
  key: String,

  npl: String,
  adj: String,
  ssg: String,
  spl: String,
  rtn: String,
  tkn: String,
  hbn: String,
  lbn: String,
  tgn: String,
  cpn: String,

  name: String,
  name_translation: String,
}

// TODO: move to a separate mod
#[derive(Debug, serde::Deserialize)]
pub struct Special {
  key: String,
  field: String,
  word: String,
  word_translation: String,
}

// TODO: move to a separate mod
#[derive(Debug, serde::Deserialize)]
pub struct Rule {
  rule: String,
  source: String,
  target: String,
  match_word: String,
  build_word: String,
  match_translation: String,
  build_translation: String,
}

#[static_init::dynamic]
static RULE_REGEX: Regex = Regex::new(r"^[A-Z]{3}$").unwrap();

type RuleContext<'a> = HashMap<&'a str, (String, String)>;

// TODO: move to a separate mod
#[derive(Debug, Default)]
pub struct RuleSet {
  // (target, rule) => (source, word_transformer, translation_transformer)
  rules: HashMap<(String, String), (String, transformer::Transformer, transformer::Transformer)>,
  // (key, field) => (word, word_translation)
  specials: HashMap<(String, String), (String, String)>,
}

impl RuleSet {
  pub fn insert_rule(&mut self, rule: Rule) {
    let Rule {
      rule,
      source,
      target,
      match_word: word_match,
      build_word: word_generate,
      match_translation: translation_match,
      build_translation: translation_generate,
    } = rule;
    self.rules.insert(
      (target, rule),
      (
        source,
        transformer::Transformer::new(
          placeholder::Placeholder::new(&word_match),
          placeholder::Placeholder::new(&word_generate),
        ),
        transformer::Transformer::new(
          placeholder::Placeholder::new(&translation_match),
          placeholder::Placeholder::new(&translation_generate),
        ),
      ),
    );
  }

  pub fn insert_special(&mut self, special: Special) {
    let Special {
      key,
      field,
      word,
      word_translation,
    } = special;
    self.specials.insert((key, field), (word, word_translation));
  }

  pub fn process(&self, key: &str, context: &mut RuleContext) {
    // println!("<<< {context:?}");
    for field in context.keys().cloned().collect::<Vec<&str>>() {
      let (rule, _) = context.get(field).unwrap();
      let rule = rule.to_owned();

      if rule == "IGN" || !RULE_REGEX.is_match(&rule) {
        continue;
      }

      if rule == "SPL" {
        if let Some((word, translated)) = self.specials.get(&(key.to_owned(), field.to_owned())) {
          context.insert(field, (word.to_owned(), translated.to_owned()));
        } else {
          panic!("cannot find special form for field {field:?} of {key:?}");
        }
        continue;
      }

      if let Some((source, word_transformer, translation_transformer)) =
        self.rules.get(&(field.to_owned(), rule.to_owned()))
      {
        let source = source.to_owned();
        let (word, translated) = context.get(source.as_str()).unwrap();
        let word = word_transformer.transform(word).unwrap();
        let translated = translation_transformer.transform(translated).unwrap();
        context.insert(field, (word, translated));
      } else {
        panic!("unknown rule {rule:?}");
      }
    }
    // println!(">>> {context:?}");
  }
}

#[derive(Debug, Default)]
pub struct Plants {
  pub name_singulars: dictionary::Dictionary,
  pub name_plurals: dictionary::Dictionary,
  pub seed_singulars: dictionary::Dictionary,
  pub seed_plurals: dictionary::Dictionary,
  pub root_names: dictionary::Dictionary,
  pub trunk_names: dictionary::Dictionary,
  pub heavy_branch_names: dictionary::Dictionary,
  pub light_branch_names: dictionary::Dictionary,
  pub twig_names: dictionary::Dictionary,
  pub cap_names: dictionary::Dictionary,

  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
}

impl Plants {
  fn new() -> Self {
    let mut plants = Plants::default();
    let mut rule_set: RuleSet = RuleSet::default();

    let file = File::open("./dfint-data/translations/plants-rules.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      rule_set.insert_rule(result.unwrap());
    }

    let file = File::open("./dfint-data/translations/plants-special.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      rule_set.insert_special(result.unwrap());
    }

    let file = File::open("./dfint-data/translations/plants-base.csv").unwrap();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.deserialize() {
      let PlantBase {
        key,
        npl,
        adj,
        ssg,
        spl,
        rtn,
        tkn,
        hbn,
        lbn,
        tgn,
        cpn,
        name,
        name_translation,
      } = result.unwrap();
      let mut context = RuleContext::new();
      context.insert("NSG", (name, name_translation));
      context.insert("NPL", (npl, String::new()));
      context.insert("ADJ", (adj, String::new()));
      context.insert("SSG", (ssg, String::new())); // SSG is needed by SPL
      context.insert("SPL", (spl, String::new()));
      context.insert("RTN", (rtn, String::new()));
      context.insert("TKN", (tkn, String::new()));
      context.insert("HBN", (hbn, String::new()));
      context.insert("LBN", (lbn, String::new()));
      context.insert("TGN", (tgn, String::new()));
      context.insert("CPN", (cpn, String::new()));

      rule_set.process(&key, &mut context);
      for (field, (word, translated)) in context {
        if field != "ADJ" {
          plants.nouns.insert(word.to_owned(), translated.to_owned());
        }

        let dict = match field {
          "NSG" => &mut plants.name_singulars,
          "NPL" => &mut plants.name_plurals,
          "ADJ" => &mut plants.adjectives,
          "SSG" => &mut plants.seed_singulars,
          "SPL" => &mut plants.seed_plurals,
          "RTN" => &mut plants.root_names,
          "TKN" => &mut plants.trunk_names,
          "HBN" => &mut plants.heavy_branch_names,
          "LBN" => &mut plants.light_branch_names,
          "TGN" => &mut plants.twig_names,
          "CPN" => &mut plants.cap_names,
          field => panic!("unhandled field {field:?}"),
        };

        dict.insert(word, translated);
      }
    }

    // println!("??? {plants:#?}");

    plants
  }
}
