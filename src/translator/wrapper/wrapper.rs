#[derive(Debug, Default)]
pub struct Wrapper<'a> {
  pub prefix: &'a str,
  pub suffix: &'a str,
}

pub type UnwrapResult<'a> = (&'a str, Wrapper<'a>);

impl<'a> Wrapper<'a> {
  pub fn unwrap(remaining: &'a str, prefix_len: usize, suffix_len: usize) -> UnwrapResult<'a> {
    let mut remaining = remaining;
    let prefix;
    let suffix;

    (prefix, remaining) = remaining.split_at(prefix_len);
    (remaining, suffix) = remaining.split_at(remaining.len() - suffix_len);

    (remaining, Self { prefix, suffix })
  }

  pub fn wrap(&self, text: &str) -> String {
    format!("{}{}{}", self.prefix, text, self.suffix)
  }
}
