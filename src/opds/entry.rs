use super::{link::OpdsLink, StumpAuthor};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OpdsEntry {
    id: String,
    updated: String,
    title: String,
    content: Option<String>,
    authors: Option<Vec<StumpAuthor>>,
    links: Vec<OpdsLink>,
}

impl OpdsEntry {
    pub fn new(
        id: String,
        updated: DateTime<FixedOffset>,
        title: String,
        content: Option<String>,
        authors: Option<Vec<StumpAuthor>>,
        links: Option<Vec<OpdsLink>>,
    ) -> Self {
        let links = match links {
            Some(links) => links,
            None => vec![],
        };

        Self {
            id,
            updated: updated.to_rfc3339(),
            title,
            content,
            authors,
            links,
        }
    }

    fn get_content(&self) -> Option<String> {
        self.content
            .as_ref()
            .map(|content| content.clone().replace('\n', "<br/>"))
    }
}
