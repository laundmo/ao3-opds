use crate::{XmlResult, XmlWriter};

use super::{link::OpdsLink, util};
use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct OpdsEntry {
    id: String,
    updated: DateTime<FixedOffset>,
    title: String,
    content: Option<String>,
    authors: Option<Vec<String>>,
    links: Vec<OpdsLink>,
}

impl OpdsEntry {
    pub fn new(
        id: String,
        updated: DateTime<FixedOffset>,
        title: String,
        content: Option<String>,
        authors: Option<Vec<String>>,
        links: Option<Vec<OpdsLink>>,
    ) -> Self {
        let links = match links {
            Some(links) => links,
            None => vec![],
        };

        Self {
            id,
            updated,
            title,
            content,
            authors,
            links,
        }
    }

    pub fn write(&self, writer: &mut XmlWriter) -> XmlResult {
        writer
            .create_element("entry")
            .write_inner_content(|writer| {
                util::write_elem("title", &self.title, writer)?;
                util::write_elem("id", &self.id, writer)?;
                util::write_elem("updated", &self.updated.to_rfc3339(), writer)?;

                if let Some(content) = self.get_content() {
                    util::write_elem("content", content.as_str(), writer)?;
                } else {
                    writer.create_element("content").write_empty()?;
                }

                if let Some(authors) = &self.authors {
                    writer
                        .create_element("author")
                        .write_inner_content(|writer| {
                            for author in authors {
                                util::write_elem("name", &author, writer)?;
                            }
                            Ok(())
                        })?;
                }

                for link in &self.links {
                    link.write(writer)?;
                }

                Ok(())
            })
            .expect("failure");

        Ok(())
    }

    fn get_content(&self) -> Option<String> {
        self.content
            .as_ref()
            .map(|content| content.clone().replace('\n', "<br/>"))
    }
}
