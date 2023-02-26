use color_eyre::Result;
use std::env;

use crate::ao3::{History, Session};

mod ao3;
use opds::OpdsFeed;
use poem::{
    get, handler, http::StatusCode, listener::TcpListener, IntoResponse, Response, Route, Server,
};
use quick_xml::Writer;
use std::io::Cursor;

use crate::opds::{OpdsEntry, OpdsLink, OpdsLinkRel, OpdsLinkType};
mod opds;

pub type XmlWriter = Writer<Cursor<Vec<u8>>>;
pub type XmlResult = std::result::Result<(), quick_xml::Error>;

impl IntoResponse for OpdsFeed {
    fn into_response(self) -> Response {
        match self.build() {
            Ok(xml) => Response::builder()
                .status(StatusCode::OK)
                .header(
                    "Content-Type",
                    "application/atom+xml;profile=opds-catalog;kind=navigation",
                )
                .body(xml)
                .into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(err.to_string())
                .into_response(),
        }
    }
}

async fn history_page() -> Result<History> {
    let session = Session::new()?;
    let session = session.login("laundmo", &env::var("AO3_PW")?).await?;
    History::new(&session, 1).await
}

#[handler]
async fn hello() -> OpdsFeed {
    let history = history_page().await;
    if let Err(err) = history {
        println!("{:?}", err);
    }
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
    OpdsFeed::new(
        "1".to_string(),
        "Test Feed".to_string(),
        Some(vec![link]),
        vec![entry],
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    let app = Route::new().at("/hello", get(hello));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await?;
    Ok(())
}
