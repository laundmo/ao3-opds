use color_eyre::{eyre::eyre, Result};
use std::{env, num::NonZeroUsize, sync::Arc};

use crate::ao3::{AuthorizedSession, HistoryPage, Session};
use tokio::sync::Mutex;

use moka::future::Cache;
use opds::{feed, OpdsFeed};
use poem::{
    error::ResponseError,
    get, handler,
    http::{HeaderMap, HeaderValue, StatusCode},
    listener::TcpListener,
    middleware::AddData,
    web::{Data, Query},
    Endpoint, EndpointExt, IntoResponse, Response, Result as WebResult, Route, Server,
};
use quick_xml::{se, Writer};
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

use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
}

fn headers() -> WebResult<HeaderMap> {
    let mut headers = HeaderMap::new();
    let header = "application/atom+xml;profile=opds-catalog;kind=navigation"
        .parse()
        .map_err(|_| EyreError::from(eyre!("Failed to parse header")))?;
    headers.insert("Content-Type", header);
    Ok(headers)
}

#[handler]
async fn history_feed(
    Query(Pagination { page }): Query<Pagination>,
    data: Data<&Ao3Cache>,
) -> WebResult<(HeaderMap, String)> {
    if !data.history_page_cache.contains_key(&page) {
        let a = HistoryPage::new(&data.session, page)
            .await
            .map_err(EyreError::from)?;
        data.history_page_cache.insert(page, Arc::new(a)).await;
    }

    let a = data
        .history_page_cache
        .get(&page)
        .expect("should be unreachable because cache is populated beforehand");
    Ok((
        headers()?,
        se::to_string::<OpdsFeed>(&a.into())
            .map_err(|_| EyreError::from(eyre!("could not serialise")))?,
    ))
}

#[derive(Clone)]
struct Ao3Cache {
    session: AuthorizedSession,
    history_page_cache: Cache<usize, Arc<HistoryPage>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    let session = Session::new()?;
    let session = session.login("laundmo", &env::var("AO3_PW")?).await?;
    let cache = Ao3Cache {
        session,
        history_page_cache: Cache::new(100),
    };

    let app = Route::new()
        .at("/opds/v1.2/history", get(history_feed))
        .data(cache);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await?;
    Ok(())
}
