pub mod types;
pub use types::Artist;

use crate::Discogs;
use std::error::Error;

impl Artist {
    // https://www.discogs.com/developers/#page:database,header:database-artist
    pub async fn get(discogs: &Discogs, id: i32) -> Result<Artist, Box<dyn Error>> {
        let url = format!("{}/artists/{}", discogs.api_endpoint, id);
        let response = discogs.make_request(&url).await?;
        // FIXME: handle response.status

        Ok(response.json().await?)
    }
}
