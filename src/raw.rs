use crate::cxxstring::CxxString;

pub fn deref<T>(addr: usize) -> T {
  unsafe { (addr as *const T).read() }
}

pub fn deref_string(addr: usize) -> String {
  unsafe { String::from_utf8_lossy(CxxString::from_ptr(addr as *const u8).to_bytes_without_nul()).into_owned() }
}
