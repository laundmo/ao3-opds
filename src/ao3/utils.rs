use chrono::{DateTime, FixedOffset, Utc};
use color_eyre::{eyre::eyre, Result};
use lazy_static::lazy_static;
use scraper::{ElementRef, Selector};

pub(crate) fn select_next<'a>(
    html: &'a ElementRef<'a>,
    selector: &'a str,
) -> Result<ElementRef<'a>> {
    let sel = Selector::parse(selector).or(Err(eyre!("Could not parse selector")))?;
    let result = html
        .select(&sel)
        .next()
        .ok_or_else(|| eyre!("No element found with {}", selector))?;
    Ok(result)
}

pub(crate) fn select_next_str<'a>(html: &'a ElementRef<'a>, selector: &'a str) -> Result<String> {
    Ok(select_next(html, selector)?
        .text()
        .next()
        .ok_or_else(|| eyre!("No text found"))?
        .to_string())
}

pub(crate) fn select_string<'a>(html: &'a ElementRef<'a>, selector: &'a str) -> Result<String> {
    Ok(select_next(html, selector)?.text().collect::<String>())
}

pub(crate) fn select_int<'a>(html: &'a ElementRef<'a>, selector: &'a str) -> Result<i32> {
    Ok(select_string(html, selector)?.replace(',', "").parse()?)
}

pub(crate) fn select_next_attr<'a>(
    html: &'a ElementRef<'a>,
    selector: &'a str,
    attr: &'a str,
) -> Result<String> {
    Ok(select_next(html, selector)?
        .value()
        .attr(attr)
        .ok_or_else(|| eyre!("attribute {} not found", attr))?
        .to_string())
}

pub(crate) fn select_all<'a>(html: &'a ElementRef<'a>, selector: &'a str) -> Vec<ElementRef<'a>> {
    let selector = Selector::parse(selector).unwrap();
    let result: Vec<_> = html.select(&selector).collect();
    result
}

lazy_static! {
    pub(crate) static ref DT_DEFAULT: DateTime<FixedOffset> = DateTime::<Utc>::MIN_UTC.into();
}

pub(crate) fn ao3_dt_parse(s: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(s, "%d %b %Y").unwrap_or(*DT_DEFAULT)
}
