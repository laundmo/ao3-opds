use color_eyre::{eyre::eyre, Result};
use scraper::{Html, Selector};

use reqwest::{Client, ClientBuilder, Url};

pub(crate) struct Session {
    client: Client,
}
pub(crate) struct AuthorizedSession {
    client: Client,
    username: String,
    password: String,
}

impl Session {
    const BASE_URL: &'static str = "https://archiveofourown.org";

    pub(crate) fn login_url() -> Url {
        let mut url = Url::parse(Self::BASE_URL).unwrap();
        url.set_path("/users/login");
        url
    }

    pub(crate) fn new() -> Result<Self> {
        let client = ClientBuilder::new().cookie_store(true).build()?;
        Ok(Self { client })
    }

    pub(crate) async fn login(self, username: &str, password: &str) -> Result<AuthorizedSession> {
        let authenticity_token = self.get_authenticiry_token().await?;
        let payload = [
            ("user[login]", username),
            ("user[password]", password),
            ("authenticity_token", &authenticity_token),
        ];
        let res = self
            .client
            .post(Session::login_url())
            .form(&payload)
            .send()
            .await?;
        dbg!(res.url());
        if res.url().as_str() != Session::login_url().as_str() {
            Ok(AuthorizedSession {
                client: self.client,
                username: username.to_string(),
                password: password.to_string(),
            })
        } else {
            // println!("{:?}", res.text().await?);
            Err(eyre!("Invalid username or password"))
        }
    }

    async fn get_authenticiry_token(&self) -> Result<String, color_eyre::Report> {
        let body = self
            .client
            .get(Session::login_url())
            .send()
            .await?
            .text()
            .await?;
        let doc = Html::parse_document(&body);
        let selector = Selector::parse(r#"input[name="authenticity_token"]"#).unwrap();
        Ok(match doc.select(&selector).next() {
            Some(input) => input
                .value()
                .attr("value")
                .ok_or(eyre!("Issue getting authenticity token"))?
                .to_string(),
            None => "".to_string(),
        })
    }
}

impl AuthorizedSession {
    pub(crate) fn subscriptions_url(user: &str, page: usize) -> Url {
        let mut url = Url::parse(Session::BASE_URL).unwrap();
        url.set_path(&("/users/".to_string() + user + "/subscriptions"));
        url.set_query(Some(&format!("page={}", page)));
        url
    }
    pub(crate) fn bookmarks_url(user: &str, page: usize) -> Url {
        let mut url = Url::parse(Session::BASE_URL).unwrap();
        url.set_path(&("/users/".to_string() + user + "/bookmarks"));
        url.set_query(Some(&format!("page={}", page)));
        url
    }

    pub(crate) fn history_url(user: &str, page: usize) -> Url {
        let mut url = Url::parse(Session::BASE_URL).unwrap();
        url.set_path(&("/users/".to_string() + user + "/readings"));
        url.set_query(Some(&format!("page={}", page)));
        url
    }

    pub(crate) async fn get_history_page(&self, page: usize) -> Result<Html> {
        let body = self
            .client
            .get(Self::history_url(&self.username, page))
            .send()
            .await?
            .text()
            .await?;
        Ok(Html::parse_document(&body))
    }
}
