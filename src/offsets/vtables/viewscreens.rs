use crate::utils;

use super::super::types;

#[static_init::dynamic]
pub static VIEWSCREENS: types::VTables = {
  let mut ret = types::VTables::default();
  utils::load_csv(
    utils::offsets_path("vtables_viewscreen.csv"),
    |platform_offsets: types::PlatformSpecificOffsets| {
      let (name, offset) = platform_offsets.pair();
      ret.insert(utils::parse_hex_as_usize(&offset).unwrap(), name);
    },
  );
  ret
};
