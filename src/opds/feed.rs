use std::io::Cursor;

use super::{
    entry::OpdsEntry,
    link::{OpdsLink, OpdsLinkRel, OpdsLinkType},
    util,
};
use anyhow::Result;
use quick_xml::Writer;

#[derive(Debug)]
pub struct OpdsFeed {
    pub id: String,
    pub title: String,
    pub entries: Vec<OpdsEntry>,
    pub links: Option<Vec<OpdsLink>>,
}

impl OpdsFeed {
    pub fn new(
        id: String,
        title: String,
        links: Option<Vec<OpdsLink>>,
        entries: Vec<OpdsEntry>,
    ) -> Self {
        Self {
            id,
            title,
            entries,
            links,
        }
    }

    pub fn paginated<T>(
        id: &str,
        title: &str,
        href_postfix: &str,
        data: Vec<T>,
        page: i64,
        count: i64,
    ) -> OpdsFeed
    where
        OpdsEntry: From<T>,
    {
        (
            id.to_string(),
            title.to_string(),
            href_postfix.to_string(),
            data,
            page,
            count,
        )
            .into()
    }

    /// Build an xml string from the feed.
    pub fn build(&self) -> Result<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        writer
            .create_element("feed")
            .with_attribute(("xmlns", "http://www.w3.org/2005/Atom"))
            .with_attribute(("xmlns:opds", "http://opds-spec.org/2010/catalog"))
            .write_inner_content(|writer| {
                util::write_elem("id", &self.id, writer)?;
                util::write_elem("title", &self.title, writer)?;
                util::write_elem("updated", &chrono::Utc::now().to_rfc3339(), writer)?;

                if let Some(links) = &self.links {
                    for link in links {
                        link.write(writer)?;
                    }
                }

                for entry in &self.entries {
                    entry.write(writer)?;
                }
                Ok(())
            })?;

        Ok(String::from_utf8(writer.into_inner().into_inner())?)
    }
}

impl<T> From<(String, String, String, Vec<T>, i64, i64)> for OpdsFeed
where
    OpdsEntry: From<T>,
{
    fn from(tuple: (String, String, String, Vec<T>, i64, i64)) -> OpdsFeed {
        let (id, title, href_postfix, data, page, count) = tuple;

        let entries = data.into_iter().map(OpdsEntry::from).collect::<Vec<_>>();

        let mut links = vec![
            OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::ItSelf,
                href: format!("/opds/v1.2/{}", href_postfix),
            },
            OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::Start,
                href: "/opds/v1.2/catalog".into(),
            },
        ];

        if page > 0 {
            links.push(OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::Previous,
                href: format!("/opds/v1.2/{}?page={}", href_postfix, page - 1),
            });
        }

        // TODO: this 20.0 is the page size, which I might make dynamic for OPDS routes... but
        // not sure..
        let total_pages = (count as f32 / 20.0).ceil() as u32;

        if page < total_pages as i64 && entries.len() == 20 {
            links.push(OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::Next,
                href: format!("/opds/v1.2/{}?page={}", href_postfix, page + 1),
            });
        }

        OpdsFeed::new(id, title, Some(links), entries)
    }
}
