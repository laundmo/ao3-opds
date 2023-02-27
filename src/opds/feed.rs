use std::io::Cursor;

use super::{
    entry::OpdsEntry,
    link::{OpdsLink, OpdsLinkRel, OpdsLinkType},
    util,
};
use color_eyre::Result;
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
        page: usize,
        has_next: bool,
        has_prev: bool,
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
            has_next,
            has_prev,
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

impl<T> From<(String, String, String, Vec<T>, usize, bool, bool)> for OpdsFeed
where
    OpdsEntry: From<T>,
{
    fn from(tuple: (String, String, String, Vec<T>, usize, bool, bool)) -> OpdsFeed {
        let (id, title, href_postfix, data, page, has_next, has_previous) = tuple;

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

        if has_previous {
            links.push(OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::Previous,
                href: format!("/opds/v1.2/{}?page={}", href_postfix, page - 1),
            });
        }

        if has_next {
            links.push(OpdsLink {
                link_type: OpdsLinkType::Navigation,
                rel: OpdsLinkRel::Next,
                href: format!("/opds/v1.2/{}?page={}", href_postfix, page + 1),
            });
        }

        OpdsFeed::new(id, title, Some(links), entries)
    }
}
