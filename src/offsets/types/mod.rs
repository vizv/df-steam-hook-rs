use std::collections::BTreeMap;

mod checksums;
pub use checksums::*;

mod os_specific_offsets;
pub use os_specific_offsets::*;

mod platform_specific_offsets;
pub use platform_specific_offsets::*;

mod contexts;
pub use contexts::*;

pub type Offsets = BTreeMap<String, usize>;
pub type ModuleOffsets = BTreeMap<String, (String, usize)>;
pub type VTables = BTreeMap<usize, String>;
