mod pipeline;
pub use pipeline::*;

mod word;
pub use word::deprecated_match_dictionaries;
pub use word::deprecated_match_dictionary;
pub use word::word_matcher;
pub use word::WordMatch;

mod wildcard;
pub use wildcard::wildcard_matcher;
