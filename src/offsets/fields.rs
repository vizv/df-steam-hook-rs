use crate::utils;

use super::types;

#[static_init::dynamic]
pub static FIELDS: types::Offsets = {
  let mut ret = types::Offsets::default();
  utils::load_csv(
    utils::offsets_path("fields.csv"),
    |os_offsets: types::OsSpecificOffsets| {
      let (name, offset) = os_offsets.pair();
      let mut offset = utils::parse_hex_as_usize(&offset).unwrap();

      let mut parts: Vec<&str> = name.split('.').collect();
      let _ = parts.pop().unwrap();
      if parts.len() > 1 {
        let prefix = parts.join(".");
        offset += ret.get(&prefix).unwrap();
      }

      ret.insert(name, offset);
    },
  );
  ret
};
