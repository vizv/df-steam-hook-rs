use std::collections::HashMap;

use crate::{translator::data::rules, utils};

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

#[derive(Debug, serde::Deserialize)]
struct PlantGrowthBase {
  key: String,

  nsg: String,
  npl: String,
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
  pub growth_name_singulars: dictionary::Dictionary,
  pub growth_name_plurals: dictionary::Dictionary,

  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
}

impl Plants {
  fn new() -> Self {
    let mut plants = Plants::default();
    let mut plants_rule_set = rules::RuleSet::default();
    let mut base_names = HashMap::<String, (String, String)>::default();
    utils::load_csv(utils::translations_path("plants-rules.csv"), |rule: rules::Rule| {
      plants_rule_set.insert_rule(rule);
    });
    utils::load_csv(
      utils::translations_path("plants-special.csv"),
      |special: rules::Special| {
        plants_rule_set.insert_special(special);
      },
    );
    utils::load_csv(
      utils::translations_path("plants-base.csv"),
      |PlantBase {
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
       }| {
        base_names.insert(key.to_owned(), (name.to_owned(), name_translation.to_owned()));

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

        plants_rule_set.process(&key, &mut context);
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
      },
    );
    let mut plants_growth_rule_set = rules::RuleSet::default();
    utils::load_csv(
      utils::translations_path("plants-growth-rules.csv"),
      |rule: rules::Rule| {
        plants_growth_rule_set.insert_rule(rule);
      },
    );
    utils::load_csv(
      utils::translations_path("plants-growth-special.csv"),
      |special: rules::Special| {
        plants_growth_rule_set.insert_special(special);
      },
    );
    utils::load_csv(
      utils::translations_path("plants-growth-base.csv"),
      |PlantGrowthBase { key, nsg, npl }| {
        let mut key_parts: Vec<&str> = key.split(':').collect();
        key_parts.pop();
        let base_name = base_names.get(&key_parts.join(":")).unwrap().to_owned();

        let mut context = rules::RuleContext::new();
        context.insert("NBS", base_name);
        context.insert("NSG", (nsg, String::new()));
        context.insert("NPL", (npl, String::new()));

        plants_growth_rule_set.process(&key, &mut context);
        for (field, (word, translated)) in context {
          if field != "NBS" {
            plants.nouns.insert(word.to_owned(), translated.to_owned());
          }

          let dict = match field {
            "NBS" => continue,
            "NSG" => &mut plants.growth_name_singulars,
            "NPL" => &mut plants.growth_name_plurals,
            field => panic!("unhandled field {field:?}"),
          };

          dict.insert(word, translated);
        }
      },
    );
    plants
  }
}
