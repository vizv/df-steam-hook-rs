pub fn get(string: &str) -> Option<String> {
  if string.starts_with(' ') || string.ends_with(' ') || string.contains("  ") {
    return None;
  }

  let (string, designation_wrapper) = super::wrapper::unwrap_designation(string);
  let (string, item_count_wrapper) = super::wrapper::unwrap_item_count(string);

  super::lookup::TOP.get(string).map(|text| item_count_wrapper.wrap(&text)).map(|text| designation_wrapper.wrap(&text))
}
