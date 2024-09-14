pub type MatchResult<'a> = (&'a str, String);
pub type MatchResults<'a> = Vec<MatchResult<'a>>;
pub type MatchFn<'a> = Box<dyn Fn(&'a str) -> MatchResults + 'a>;
