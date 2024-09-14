use raw::cxxstring_as_bytes;

use crate::encodings;

pub fn deref<T>(addr: usize) -> T {
  // TODO: avoid copy
  unsafe { (addr as *const T).read() }
}

pub fn deref_string(addr: usize) -> String {
  let bytes = cxxstring_as_bytes(addr);
  let result: Vec<u8> =
    bytes.into_iter().flat_map(|&byte| encodings::cp437::CP437_TO_UTF8_BYTES[byte as usize].to_owned()).collect();
  String::from_utf8_lossy(&result).into_owned()
}
