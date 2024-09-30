mod translated_segment;
pub use translated_segment::*;
mod lookup_table;
pub use lookup_table::*;
mod lookup_tree;
pub use lookup_tree::*;
mod top;
pub use top::*;

pub fn get(string: &str) -> Option<String> {
  if string.starts_with(' ') || string.ends_with(' ') || string.contains("  ") {
    return None;
  }

  TOP.get(string)
}
