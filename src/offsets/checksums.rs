use crate::utils;

use super::types;

#[static_init::dynamic]
pub static CHECKSUMS: types::Checksums = {
  let mut ret = types::Checksums::default();
  utils::load_csv(
    utils::offsets_path("checksums.csv"),
    |types::Checksum { os, platform, checksum }| {
      // TODO: add checksum for linux-steam
      if checksum.is_empty() {
        return;
      }

      let checksums = ret.entry(os).or_default();
      let checksum = utils::parse_hex_as_u32(&checksum).unwrap();
      checksums.insert(checksum, platform);
    },
  );
  ret
};
