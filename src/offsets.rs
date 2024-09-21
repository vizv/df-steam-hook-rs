#[cfg(target_os = "linux")]
use std::{
  collections::BTreeMap,
  io::{BufRead, BufReader},
};

#[static_init::dynamic]
pub static OFFSETS: Offsets = Offsets::new();

#[derive(Debug, Default)]
pub struct Offsets {
  #[cfg(target_os = "linux")]
  g_src_maps: BTreeMap<u64, (u64, u64)>,
  pub self_base: usize,
}

impl Offsets {
  fn new() -> Self {
    let mut offsets = Offsets::default();

    #[cfg(target_os = "linux")]
    {
      let file = std::fs::File::open("/proc/self/maps").unwrap();
      let reader = BufReader::new(file);

      for line in reader.lines() {
        let line = line.unwrap();
        if !line.ends_with("libg_src_lib.so") {
          continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(range) = parts.get(0) {
          let addresses: Vec<&str> = range.split('-').collect();
          if addresses.len() != 2 {
            continue;
          }

          let start = addresses[0];
          let end = addresses[1];
          if let Ok(start) = u64::from_str_radix(start, 16) {
            if let Ok(end) = u64::from_str_radix(end, 16) {
              if let Some(&offset) = parts.get(2) {
                if let Ok(offset) = u64::from_str_radix(offset, 16) {
                  let length = end - start;
                  offsets.g_src_maps.insert(offset, (start, length));
                }
              }
            }
          }
        }
      }
    }

    #[cfg(target_os = "windows")]
    {
      offsets.self_base = unsafe { winapi::um::libloaderapi::GetModuleHandleW(std::ptr::null()) as usize };
    }

    offsets
  }

  pub fn get(&self, module: &str, offset: usize) -> usize {
    if module == "self" {
      return self.self_base + offset;
    }

    #[cfg(target_os = "linux")]
    if module == "libg_src_lib.so" {
      for (base, (start, len)) in &self.g_src_maps {
        let end = base + len;
        if offset < end as usize {
          return offset - *base as usize + *start as usize;
        }
      }
    }

    0
  }

  pub fn resolve(&self, address: usize) -> Option<(&'static str, usize)> {
    if address < 0xffffffff {
      return Some(("self", address - self.self_base));
    }

    #[cfg(target_os = "linux")]
    {
      for (base, (start, len)) in &self.g_src_maps {
        let end = start + len;
        if address >= *start as usize && address < end as usize {
          return Some(("libg_src_lib.so", address - *start as usize + *base as usize));
        }
      }
    }

    None
  }
}
