mod pipeline;

mod word;
pub use word::deprecated_match_dictionaries;
pub use word::deprecated_match_dictionary;
pub use word::match_dictionary;
pub use word::WordMatch;

mod wildcard;
pub use wildcard::match_wildcard_table;
