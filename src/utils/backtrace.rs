use crate::offsets;

pub fn backtrace() -> String {
  let mut addresses = Vec::<String>::default();
  backtrace::trace(|frame| {
    let address = frame.ip() as usize;
    if address == 0 {
      return false;
    }

    if let Some((module, offset)) = offsets::OFFSETS.resolve(address) {
      if let Some(offset) = match module {
        "self" => Some(format!("S{offset:x}")),
        "libg_src_lib.so" => Some(format!("G{offset:x}")),
        _ => None,
      } {
        addresses.push(offset);
      }
    }

    true
  });
  addresses.join("/")
}
