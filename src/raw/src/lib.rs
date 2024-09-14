use core::slice;

extern "C" {
  fn string_create() -> usize;
  fn string_create_n_chars(len: usize, ch: u8) -> usize;
  fn string_delete(ptr: usize);
  fn string_len(ptr: usize) -> usize;
  fn string_to_str(ptr: usize) -> *const u8;
}

pub fn new_cxxstring() -> usize {
  unsafe { string_create() }
}

pub fn new_cxxstring_n_chars(len: usize, ch: u8) -> usize {
  unsafe { string_create_n_chars(len, ch) }
}

pub fn delete_cxxstring(addr: usize) {
  unsafe { string_delete(addr) };
}

pub fn cxxstring_as_bytes<'a>(addr: usize) -> &'a [u8] {
  unsafe { slice::from_raw_parts(string_to_str(addr), string_len(addr)) }
}
