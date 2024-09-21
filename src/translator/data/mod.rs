mod alignment;
pub use alignment::Alignment;

mod dictionary;
mod wildcard_table;
pub use dictionary::Dictionary;
pub use wildcard_table::WildcardTable;

mod placeholder;
mod transformer;
mod rules;

mod interfaces;
pub use interfaces::INTERFACES;

mod help;
pub use help::HELP;

// static strings
mod skill_levels;
mod skill_names;
pub use skill_levels::SKILL_LEVELS;
pub use skill_names::SKILL_NAMES;

// plant-related strings
mod plants;
pub use plants::PLANTS;

// raw files strings
mod items;
mod materials;
mod materials_templates;
pub use items::ITEMS;
pub use materials::MATERIALS;
pub use materials_templates::MATERIALS_TEMPLATES;

// mega directory
mod mega;
pub use mega::MEGA;

// TODO: remove legacy dictionary completely
mod legacy;
pub use legacy::*;
