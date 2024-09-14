#[derive(Debug, Default)]
pub struct Placeholder {
  prefix: String,
  suffix: String,
}

impl Placeholder {
  pub fn new(placeholder: &str) -> Self {
    let (prefix, suffix) = placeholder.split_once("{}").unwrap();
    let prefix = prefix.to_owned();
    let suffix = suffix.to_owned();

    Self { prefix, suffix }
  }

  pub fn unwrap<'a>(&self, string: &'a str) -> Option<&'a str> {
    string.strip_prefix(&self.prefix)?.strip_suffix(&self.suffix)
  }

  pub fn wrap(&self, string: &str) -> String {
    vec![&self.prefix, string, &self.suffix].concat()
  }
}
