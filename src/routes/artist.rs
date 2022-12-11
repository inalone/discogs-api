use crate::{Discogs, Image};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct Member {
    pub active: bool,
    pub id: i32,
    pub name: String,
    pub resource_url: String,
}

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
    pub images: Vec<Image>,
    pub members: Vec<Member>,
}

impl Artist {
    // https://www.discogs.com/developers/#page:database,header:database-artist
    pub async fn get(discogs: &Discogs, id: i32) -> Result<Artist, Box<dyn Error>> {
        let url = format!("{}/artists/{}", discogs.api_endpoint, id);
        let response = discogs.make_request(&url).await?;
        // FIXME: handle response.status

        Ok(response.json().await?)
    }
}
