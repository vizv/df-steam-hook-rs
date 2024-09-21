use crate::offsets;

pub fn backtrace() -> String {
  let mut addresses = Vec::<String>::default();
  backtrace::trace(|frame| {
    let ip = frame.ip() as usize;
    if ip == 0 {
      return false;
    }
    let fp = frame.symbol_address() as usize;

    for (i, address) in vec![ip, fp].into_iter().enumerate() {
      let prefix = if i == 0 { "^" } else { "" };
      if let Some((module, offset)) = offsets::OFFSETS.resolve(address) {
        if let Some(offset) = match module {
          "self" => Some(format!("{prefix}{offset:x}")),
          "libg_src_lib.so" => Some(format!("{prefix}@{offset:x}")),
          _ => None,
        } {
          addresses.push(offset);
        }
      }
    }

    true
  });
  addresses.join("/")
}
