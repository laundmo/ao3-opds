use color_eyre::Result;
use std::env;

use crate::ao3::{History, Session};

mod ao3;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install()?;
    let session = Session::new()?;
    let session = session.login("laundmo", &env::var("AO3_PW")?).await?;
    dbg!(History::new(&session, 1).await?);
    Ok(())
}
