use chrono::{DateTime, FixedOffset};
use color_eyre::{eyre::eyre, Result};
use scraper::ElementRef;

use super::utils::*;

#[derive(Debug)]
pub(crate) struct Author {
    name: String,
}

impl Author {
    pub(crate) fn from_element(element: &ElementRef) -> Result<Author> {
        Ok(Author {
            name: element
                .text()
                .next()
                .ok_or_else(|| eyre!("no author text found"))?
                .to_string(),
        })
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
    author: Author,
    title: String,
    uri: String,
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
        let author_element = select_next(&heading, r#"a[rel="author"]"#)?;
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
            author: Author::from_element(&author_element)?,
            title,
            uri,
            tags: Tags::from_element(&tags_element)?,
            summary: select_next_str(element, "blockquote.summary")?,
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
