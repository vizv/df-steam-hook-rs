use std::fs::File;

use crate::translator::data::rules;

use super::dictionary;

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
    let mut rule_set: rules::RuleSet = rules::RuleSet::default();

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
      let mut context = rules::RuleContext::new();
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
