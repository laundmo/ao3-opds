use chrono::{DateTime, FixedOffset};
use color_eyre::Result;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::ElementRef;

use crate::opds::{OpdsEntry, OpdsFeed};

use super::{session::AuthorizedSession, utils::*, Work};

#[derive(Debug)]
enum Changed {
    Latest,
    Minor,
    Updated,
    Unknown(String),
}
#[derive(Debug)]
pub(crate) struct HistoryWork {
    work: Work,
    last_visited: DateTime<FixedOffset>,
    changed: Changed,
    visited: i32,
}

lazy_static! {
    static ref HISTORY_RE: Regex =
        Regex::new(r#"Last visited: (.+)\n\n.+\((.+)\)\n\n.+Visited (.+)\n"#).unwrap();
}

impl HistoryWork {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Self> {
        let visited = select_next(element, "div.user > h4")?;
        let visited = visited.text().collect::<String>();
        let caps = HISTORY_RE.captures(&visited).unwrap();

        let last_visited = caps
            .get(1)
            .map_or(*DT_DEFAULT, |m| ao3_dt_parse(m.as_str()));

        let changed = match caps.get(2).map_or("", |m| m.as_str()) {
            "Latest version." => Changed::Latest,
            "Minor edits made since then." => Changed::Minor,
            "Update available." => Changed::Updated,
            other => Changed::Unknown(other.to_string()),
        };

        let visited = caps.get(3).map_or(0, |m| {
            let s = m.as_str();
            if s == "once" {
                1
            } else {
                s.split_once(' ').map_or(0, |s| {
                    s.0.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse {}", s.1))
                })
            }
        });

        Ok(HistoryWork {
            work: Work::from_element(element)?,
            last_visited,
            changed,
            visited,
        })
    }
}

impl From<&HistoryWork> for OpdsEntry {
    fn from(value: &HistoryWork) -> Self {
        (&value.work).into() // TODO: maybe add to the OpdsEntry when doing this
    }
}

#[derive(Debug)]
pub(crate) struct HistoryPage {
    history: Vec<HistoryWork>,
    page: usize,
    has_next: bool,
    has_prev: bool,
}

impl HistoryPage {
    pub(crate) fn from_element(element: &ElementRef, page: usize) -> Result<HistoryPage> {
        let mut history = Vec::new();

        for element in select_all(element, "ol.reading.index > li.reading.blurb") {
            let work = HistoryWork::from_element(&element)?;
            history.push(work);
        }
        let has_prev = select_next(element, "ol.pagination > li.previous > a")
            .ok()
            .is_some();
        let has_next = select_next(element, "ol.pagination > li.next > a")
            .ok()
            .is_some();

        Ok(HistoryPage {
            history,
            page,
            has_next,
            has_prev,
        })
    }

    pub(crate) async fn new(session: &AuthorizedSession, page: usize) -> Result<HistoryPage> {
        let html = session.get_history_page(page).await?;
        Self::from_element(&html.root_element(), page)
    }
}

impl From<&HistoryPage> for OpdsFeed {
    fn from(value: &HistoryPage) -> Self {
        OpdsFeed::paginated(
            &format!("history-page-{}", value.page),
            &format!("History page {}", value.page),
            "history",
            value.history.iter().collect(),
            value.page,
            value.has_next,
            value.has_prev,
        )
    }
}
