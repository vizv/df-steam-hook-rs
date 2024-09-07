mod dictionary;

pub use dictionary::Dictionary;

// static strings
mod skill_levels;
mod skill_names;

pub use skill_levels::SKILL_LEVELS;
pub use skill_names::SKILL_NAMES;

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
