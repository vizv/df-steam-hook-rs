use crate::utils;

use super::dictionary;

#[static_init::dynamic]
pub static MATERIALS_TEMPLATES: MaterialsTemplates = MaterialsTemplates::new();

#[derive(Debug, serde::Deserialize)]
struct MaterialsTemplate {
  noun_all_solid: String,
  noun_all_solid_translation: String,
  noun_solid: String,
  noun_solid_translation: String,
  noun_powder: String,
  noun_powder_translation: String,
  noun_liquid: String,
  noun_liquid_translation: String,
  noun_gas: String,
  noun_gas_translation: String,
  adjective_all_solid: String,
  adjective_all_solid_translation: String,
  adjective_solid: String,
  adjective_solid_translation: String,
  adjective_powder: String,
  adjective_powder_translation: String,
  adjective_liquid: String,
  adjective_liquid_translation: String,
  adjective_gas: String,
  adjective_gas_translation: String,
}

#[derive(Debug, Default)]
pub struct MaterialsTemplates {
  pub nouns: dictionary::Dictionary,
  pub adjectives: dictionary::Dictionary,
}

impl MaterialsTemplates {
  fn new() -> Self {
    let mut materials_templates = MaterialsTemplates::default();
    utils::load_csv(
      utils::translations_path("materials_templates.csv"),
      |MaterialsTemplate {
         noun_all_solid,
         noun_all_solid_translation,
         noun_solid,
         noun_solid_translation,
         noun_powder,
         noun_powder_translation,
         noun_liquid,
         noun_liquid_translation,
         noun_gas,
         noun_gas_translation,
         adjective_all_solid,
         adjective_all_solid_translation,
         adjective_solid,
         adjective_solid_translation,
         adjective_powder,
         adjective_powder_translation,
         adjective_liquid,
         adjective_liquid_translation,
         adjective_gas,
         adjective_gas_translation,
       }| {
        materials_templates.nouns.insert(noun_all_solid, noun_all_solid_translation);
        materials_templates.nouns.insert(noun_solid, noun_solid_translation);
        materials_templates.nouns.insert(noun_powder, noun_powder_translation);
        materials_templates.nouns.insert(noun_liquid, noun_liquid_translation);
        materials_templates.nouns.insert(noun_gas, noun_gas_translation);
        materials_templates.adjectives.insert(adjective_all_solid, adjective_all_solid_translation);
        materials_templates.adjectives.insert(adjective_solid, adjective_solid_translation);
        materials_templates.adjectives.insert(adjective_powder, adjective_powder_translation);
        materials_templates.adjectives.insert(adjective_liquid, adjective_liquid_translation);
        materials_templates.adjectives.insert(adjective_gas, adjective_gas_translation);
      },
    );
    materials_templates
  }
}
