use crate::utils;

use super::types;

#[static_init::dynamic]
pub static GLOBALS: types::ModuleOffsets = {
  let mut ret = types::ModuleOffsets::default();
  utils::load_csv(
    utils::offsets_path("globals.csv"),
    |platform_offsets: types::PlatformSpecificOffsets| {
      let (name, module_offset) = platform_offsets.pair();
      let (module, offset) = module_offset.split_once(":").unwrap();
      ret.insert(name, (module.to_owned(), utils::parse_hex_as_usize(offset).unwrap()));
    },
  );
  ret
};
