use serde::Serialize;

/// Represents an author in an OPDS feed.
#[derive(Debug, Serialize)]
pub struct StumpAuthor {
    pub name: String,
    pub uri: Option<String>,
}

impl StumpAuthor {
    pub fn new(name: String, uri: Option<String>) -> StumpAuthor {
        StumpAuthor { name, uri }
    }
}
