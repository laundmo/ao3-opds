use chrono::{DateTime, FixedOffset};
use color_eyre::{eyre::eyre, Result};
use scraper::ElementRef;

use super::utils::*;
use crate::opds::OpdsLinkRel;
use crate::opds::OpdsLinkType;
use crate::opds::{OpdsEntry, OpdsLink};

#[derive(Debug)]
pub(crate) struct Authors(Vec<String>);

impl Authors {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Authors> {
        let mut authors = Vec::new();
        for a in select_all(element, r#"a[rel="author"]"#) {
            authors.push(
                a.text()
                    .next()
                    .ok_or_else(|| eyre!("Issue parsing author"))?
                    .to_string(),
            );
        }

        Ok(Authors(authors))
    }
}

#[derive(Debug)]
pub(crate) struct Tags {
    warnings: Vec<String>,
    relationships: Vec<String>,
    characters: Vec<String>,
    freeform: Vec<String>,
}

impl Tags {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Tags> {
        let relationships: Vec<String> = {
            let mut vec = Vec::new();
            for e in select_all(element, "li.relationships") {
                vec.push(select_next_str(&e, "a")?);
            }
            vec
        };
        let characters: Vec<String> = {
            let mut vec = Vec::new();
            for e in select_all(element, "li.characters") {
                vec.push(select_next_str(&e, "a")?);
            }
            vec
        };
        let freeform: Vec<String> = {
            let mut vec = Vec::new();
            for e in select_all(element, "li.freeforms") {
                vec.push(select_next_str(&e, "a")?);
            }
            vec
        };
        Ok(Tags {
            warnings: vec![select_next_str(element, "li.warnings > strong > a")?],
            relationships,
            characters,
            freeform,
        })
    }
}

#[derive(Debug)]
pub(crate) struct SeriesRef {
    name: String,
    uri: String,
    part: i32,
}

impl SeriesRef {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Self> {
        let part = select_int(element, "li > strong")?;
        let name = select_next_str(element, "a")?;
        let uri = select_next_attr(element, "a", "href")?;

        Ok(SeriesRef { name, uri, part })
    }
}

#[derive(Debug)]
enum Chapters {
    Known(i32, i32),
    Unknown(i32),
}

#[derive(Debug)]
pub(crate) struct Work {
    authors: Authors, // TODO: Fandom!
    title: String,
    id: i64,
    tags: Tags,
    summary: String,
    series: Option<SeriesRef>,
    last_updated: DateTime<FixedOffset>,
    language: String,
    words: i32,
    chapters: Chapters,
    comments: i32,
    kudos: i32,
    bookmarks: i32,
    hits: i32,
}

impl Work {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Self> {
        let heading = select_next(element, "h4.heading")?;
        let title = select_next_str(&heading, "a")?;
        let uri = select_next_attr(&heading, "a", "href")?;
        let id = uri
            .split('/')
            .last()
            .ok_or_else(|| eyre!("could not split uri: {}", uri))?
            .parse::<i64>()?;
        let last_updated = ao3_dt_parse(&select_next_str(element, "div > p.datetime")?);

        let chapters = {
            let chapters = select_string(element, "dl.stats > dd.chapters")?;
            let (a, b) = chapters
                .split_once('/')
                .ok_or_else(|| eyre!("could not split chapter: {}", chapters))?;
            let a = a.parse()?;
            match b.parse() {
                Ok(b) => Chapters::Known(a, b),
                Err(_) => Chapters::Unknown(a),
            }
        };

        let series = select_next(element, "ul.series")
            .ok()
            .and_then(|e| SeriesRef::from_element(&e).ok());

        let tags_element = select_next(element, r#"ul.tags"#)?;
        Ok(Work {
            authors: Authors::from_element(&heading)?,
            title,
            id,
            tags: Tags::from_element(&tags_element)?,
            summary: select_string(element, "blockquote.summary")?,
            series,
            last_updated,
            language: select_next_str(element, "dl.stats > dd.language")?,
            words: select_int(element, "dl.stats > dd.words")?,
            chapters,
            comments: select_int(element, "dl.stats > dd.comments > a")?,
            kudos: select_int(element, "dl.stats > dd.kudos > a")?,
            bookmarks: select_int(element, "dl.stats > dd.bookmarks > a").unwrap_or(0),
            hits: select_int(element, "dl.stats > dd.hits")?,
        })
    }
}

impl From<&Work> for OpdsEntry {
    fn from(value: &Work) -> Self {
        let content: String = format!(
            r"[{}], [{}], [{}], [{}]\n{}",
            value.tags.warnings.first().unwrap_or(&"".to_string()),
            value.tags.relationships.first().unwrap_or(&"".to_string()),
            value.tags.characters.first().unwrap_or(&"".to_string()),
            value.tags.freeform.first().unwrap_or(&"".to_string()),
            value.summary,
        );
        Self::new(
            format!("/works/{}", value.id),
            value.last_updated,
            value.title.clone(),
            Some(content),
            Some(value.authors.0.clone()),
            Some(vec![OpdsLink::new(
                OpdsLinkType::Epub,
                OpdsLinkRel::Acquisition,
                format!("https://archiveofourown.org/downloads/{}/a.epub", value.id),
            )]),
        )
    }
}
