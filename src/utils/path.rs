use std::path::PathBuf;

const DATA_DIR: &str = "dfint-data";
const OFFSETS_DIR: &str = "offsets";

pub fn data_path(subpath: &str) -> PathBuf {
  let mut ret = PathBuf::new();
  ret.push(DATA_DIR);
  ret.push(subpath);
  ret
}

pub fn offsets_path(subpath: &str) -> PathBuf {
  let mut ret = data_path(OFFSETS_DIR);
  ret.push(subpath);
  ret
}
