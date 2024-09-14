mod word;
pub use word::WordMatch;
pub use word::match_dictionaries;
pub use word::match_dictionary;

mod wildcard;
pub use wildcard::WildcardMatch;
pub use wildcard::match_wildcard_table;
