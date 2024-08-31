use crate::cp437::CP437_TO_UTF8_BYTES;
use cxx::CxxString;

pub fn deref<T>(addr: usize) -> T {
  unsafe { (addr as *const T).read() }
}

pub fn deref_string(addr: usize) -> String {
  let bytes = unsafe { std::mem::transmute::<usize, &CxxString>(addr).as_bytes() };
  let result: Vec<u8> = bytes.into_iter().flat_map(|&byte| CP437_TO_UTF8_BYTES[byte as usize].to_owned()).collect();
  String::from_utf8_lossy(&result).into_owned()
}
