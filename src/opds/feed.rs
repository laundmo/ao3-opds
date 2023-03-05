use super::{
    entry::OpdsEntry,
    link::{OpdsLink, OpdsLinkRel, OpdsLinkType},
};
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename = "feed")]
pub struct OpdsFeed {
    #[serde(rename = "@xmlns")]
    pub xmlns: String,
    #[serde(rename = "@xmlns:opds")]
    pub xmlns_opds: String,
    pub updated: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "link")]
    pub links: Option<Vec<OpdsLink>>,
    #[serde(rename = "entry")]
    pub entries: Vec<OpdsEntry>,
}

impl OpdsFeed {
    pub fn new(
        id: String,
        title: String,
        links: Option<Vec<OpdsLink>>,
        entries: Vec<OpdsEntry>,
    ) -> Self {
        Self {
            xmlns: "http://www.w3.org/2005/Atom".to_string(),
            xmlns_opds: "http://opds-spec.org/2010/catalog".to_string(),
            updated: Utc::now().to_rfc3339(),
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
}

impl<T> From<(String, String, String, Vec<T>, usize, bool, bool)> for OpdsFeed
where
    OpdsEntry: From<T>,
{
    fn from(tuple: (String, String, String, Vec<T>, usize, bool, bool)) -> OpdsFeed {
        let (id, title, href_postfix, data, page, has_next, has_previous) = tuple;

        let entries = data.into_iter().map(OpdsEntry::from).collect::<Vec<_>>();

        let mut links = vec![
            OpdsLink::new(
                OpdsLinkType::Navigation,
                OpdsLinkRel::ItSelf,
                format!("/opds/v1.2/{}", href_postfix),
            ),
            OpdsLink::new(
                OpdsLinkType::Navigation,
                OpdsLinkRel::Start,
                "/opds/v1.2/catalog".into(),
            ),
        ];

        if has_previous {
            links.push(OpdsLink::new(
                OpdsLinkType::Navigation,
                OpdsLinkRel::Previous,
                format!("/opds/v1.2/{}?page={}", href_postfix, page - 1),
            ));
        }

        if has_next {
            links.push(OpdsLink::new(
                OpdsLinkType::Navigation,
                OpdsLinkRel::Next,
                format!("/opds/v1.2/{}?page={}", href_postfix, page + 1),
            ));
        }

        OpdsFeed::new(id, title, Some(links), entries)
    }
}

#[cfg(test)]
mod tests {
    use crate::opds::{OpdsEntry, OpdsFeed, OpdsLink, OpdsLinkRel, OpdsLinkType, StumpAuthor};

    #[test]
    fn it_works() {
        let entry = OpdsEntry::new(
            "2".to_string(),
            chrono::Utc::now().into(),
            "Test Entry".to_string(),
            Some("Test Entry Content blah blah blah".to_string()),
            Some(vec![StumpAuthor::new("Test Author".to_string(), None)]),
            Some(vec![OpdsLink::new(
                OpdsLinkType::Image,
                OpdsLinkRel::Image,
                "https://i.laundmo.com/tENe0/noLAletu89.png".to_string(),
            )]),
        );
        let link = OpdsLink::new(
            OpdsLinkType::Navigation,
            OpdsLinkRel::ItSelf,
            "/".to_string(),
        );
        let feed = OpdsFeed::new(
            "1".to_string(),
            "Test Feed".to_string(),
            Some(vec![link]),
            vec![entry],
        );
        println!("{}?", quick_xml::se::to_string(&feed).unwrap());
    }
}
