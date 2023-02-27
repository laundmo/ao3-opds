use color_eyre::{eyre::eyre, Result};
use std::env;

use crate::ao3::{HistoryPage, Session};

use opds::OpdsFeed;
use poem::{
    error::ResponseError,
    get, handler,
    http::{HeaderMap, HeaderValue, StatusCode},
    listener::TcpListener,
    web::Query,
    IntoResponse, Response, Result as WebResult, Route, Server,
};
use quick_xml::Writer;
use std::io::Cursor;

use crate::opds::{OpdsEntry, OpdsLink, OpdsLinkRel, OpdsLinkType};

mod ao3;
mod opds;

pub type XmlWriter = Writer<Cursor<Vec<u8>>>;
pub type XmlResult = std::result::Result<(), quick_xml::Error>;

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
struct EyreError {
    message: String,
}

impl From<color_eyre::Report> for EyreError {
    fn from(value: color_eyre::Report) -> Self {
        EyreError {
            message: format!("{:?}", value),
        }
    }
}

impl ResponseError for EyreError {
    fn status(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}

async fn history_page(page: usize) -> Result<HistoryPage> {
    let session = Session::new()?;
    let session = session.login("laundmo", &env::var("AO3_PW")?).await?;
    HistoryPage::new(&session, page).await
}

fn feed_to_web(feed: OpdsFeed) -> WebResult<(HeaderMap, String)> {
    let mut headers = HeaderMap::new();
    let header = "application/atom+xml;profile=opds-catalog;kind=navigation"
        .parse()
        .map_err(|_| EyreError::from(eyre!("Failed to parse header")))?;
    headers.insert("Content-Type", header);
    Ok((headers, feed.build().map_err(EyreError::from)?))
}

use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
}

#[handler]
async fn history_feed(
    Query(Pagination { page }): Query<Pagination>,
) -> WebResult<(HeaderMap, String)> {
    let history = history_page(page).await.map_err(EyreError::from)?;
    feed_to_web(history.into())
}

#[handler]
fn testfeed() -> WebResult<(HeaderMap, String)> {
    // todo
    let entry = OpdsEntry::new(
        "2".to_string(),
        chrono::Utc::now().into(),
        "Test Entry".to_string(),
        Some("Test Entry Content blah blah blah".to_string()),
        Some(vec!["testauthor".to_string()]),
        Some(vec![OpdsLink {
            href: "https://i.laundmo.com/tENe0/noLAletu89.png".to_string(),
            rel: OpdsLinkRel::Image,
            link_type: OpdsLinkType::Image,
        }]),
    );
    let link = OpdsLink {
        link_type: OpdsLinkType::Navigation,
        rel: OpdsLinkRel::ItSelf,
        href: "/".to_string(),
    };
    let feed = OpdsFeed::new(
        "1".to_string(),
        "Test Feed".to_string(),
        Some(vec![link]),
        vec![entry],
    );
    feed_to_web(feed)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    let app = Route::new().at("/opds/v1.2/history", get(history_feed));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await?;
    Ok(())
}
