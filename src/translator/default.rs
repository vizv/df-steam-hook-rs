pub fn get(string: &str) -> Option<String> {
  if string.starts_with(' ') || string.ends_with(' ') || string.contains("  ") {
    return None;
  }

  let (string, designation_wrapper) = super::wrapper::unwrap_designation(string);
  // FIXME: temporary work around for FISH
  if string.contains(", ") {
    let results = &string.split(", ").map(|string_part| get(string_part)).collect::<Vec<Option<String>>>();
    if results.iter().find(|&result| result.is_some()).is_none() {
      return None;
    }

    return Some(
      designation_wrapper.wrap(
        &string
          .split(", ")
          .map(|string_part| get(&string_part.to_lowercase()).unwrap_or(string_part.to_owned()))
          .collect::<Vec<String>>()
          .join(", "),
      ),
    );
  }

  let (string, item_count_wrapper) = super::wrapper::unwrap_item_count(string);

  super::lookup::TOP
    .get(&string.to_lowercase())
    .map(|text| item_count_wrapper.wrap(&text))
    .map(|text| designation_wrapper.wrap(&text))
}
