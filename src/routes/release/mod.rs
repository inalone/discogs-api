pub mod types;
pub use types::Release;

use crate::{Currency, Discogs};
use std::error::Error;

impl Release {
    // https://www.discogs.com/developers/#page:database,header:database-release
    pub async fn get(
        discogs: &Discogs,
        id: i32,
        currency: Currency,
    ) -> Result<Release, Box<dyn Error>> {
        let currency_str = match currency {
            Currency::Unspecified => "".to_string(),
            _ => format!("?{}", currency.to_string()),
        };
        let url = format!("{}/releases/{}{}", discogs.api_endpoint, id, currency_str);
        let response = discogs.make_request(&url).await?;
        // FIXME: handle response.status

        Ok(response.json().await?)
    }
}
