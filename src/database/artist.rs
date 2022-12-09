use crate::Discogs;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub namevariations: Vec<String>,
    pub profile: String,
    pub releases_url: String,
    pub resource_url: String,
    pub uri: String,
    pub urls: Vec<String>,
    pub data_quality: String,
    pub id: i32,
    // FIXME: add images and members
    // pub images: Vec<Image>
    // pub members
}

impl Artist {
    pub async fn get(discogs: &mut Discogs, id: i32) -> Result<Artist, Box<dyn Error>> {
        let url = format!("{}/artists/{}", discogs.api_endpoint, id);
        let response = discogs.make_request(&url).await?;
        // FIXME: handle response.status

        Ok(response.json().await?)
    }
}
