use crate::Image;
use serde::Deserialize;

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
