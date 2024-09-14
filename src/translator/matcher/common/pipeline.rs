pub type MatchResult<'a> = (&'a str, &'a str, String);
pub type MatchResults<'a> = Vec<MatchResult<'a>>;
pub type MatchFn<'a> = Box<dyn Fn(&'a str) -> MatchResults + 'a>;

pub fn match_pipeline<'a, N>(matcher: MatchFn<'a>, remaining: &'a str, next: N) -> (bool, Option<String>)
where
  N: Fn(&'a str) -> (bool, Option<String>),
{
  for (remaining, _, translated) in matcher(&remaining) {
    let (matched, next_translated_opt) = next(remaining);
    if matched {
      if let Some(next_translated) = next_translated_opt {
        return (true, Some(vec![&translated, "", &next_translated].concat()));
      } else {
        return (true, Some(translated));
      }
    }
  }

  (false, None)
}
