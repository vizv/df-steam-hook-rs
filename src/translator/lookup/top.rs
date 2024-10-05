const CSV_FILES: &[&str] = &[
  "creatures.csv",
  "plants.csv",
  "plants-growths.csv",
  "skills.csv",
  "professions.csv",
  "positions.csv",
  "materials.csv",
  "tiles.csv",
  "info-tags.csv",
  "gems.csv",
  "gems-details.csv",
  "weapons.csv",
  "armors.csv",
  "shoes.csv",
  "shields.csv",
  "helms.csv",
  "gloves.csv",
  "ammos.csv",
  "meats.csv",
  "items.csv",
  "tasks.csv",
];
const TOP_LOOKUPS: &[&str] = &[
  "CREATURE",
  "PLANT_GROWTH",
  "SKILL",
  "PROFESSION",
  "POSITION",
  "MATERIAL",
  "INFO_TAG:FLOOR", // TODO: add other types of INFO_TAG
  "ITEM",
  "TASK",
];

#[static_init::dynamic]
pub static TOP: super::LookupTree = {
  let mut ret = super::LookupTree::default();

  for &file in CSV_FILES {
    ret.load_csv(file);
  }

  for &lookup in TOP_LOOKUPS {
    ret.enable(lookup);
  }

  // ret.dump_all("");
  ret
};
