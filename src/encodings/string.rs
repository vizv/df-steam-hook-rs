pub fn bytes_to_string(bytes: &[u8]) -> String {
  let result: Vec<u8> = bytes.into_iter().flat_map(|&byte| super::cp437_byte_to_utf8_char(byte).to_owned()).collect();
  String::from_utf8_lossy(&result).into_owned()
}

pub fn read_raw_string(addr: usize) -> String {
  let bytes = raw::cxxstring_as_bytes(addr);
  bytes_to_string(bytes)
}
