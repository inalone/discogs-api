use crate::Image;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub anv: String,
    pub id: i32,
    pub join: String,
    pub name: String,
    pub resource_url: String,
    pub role: String,
    pub tracks: String,
}

#[derive(Deserialize, Debug)]
pub struct Community {
    pub contributors: Vec<Person>,
    pub data_quality: String,
    pub have: i32,
    pub rating: AverageRating,
    pub status: String,
    pub submitter: Person,
    pub want: i32,
}

#[derive(Deserialize, Debug)]
pub struct Company {
    pub catno: String,
    // TODO: make enum
    pub entity_type: String,
    pub entity_type_name: String,
    pub id: i32,
    pub name: String,
    pub resource_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Format {
    pub descriptions: Vec<String>,
    pub name: String,
    pub qty: String,
}

#[derive(Deserialize, Debug)]
pub struct Identifier {
    #[serde(rename = "type")]
    pub identifier_type: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Label {
    pub catno: String,
    pub entity_type: String,
    pub id: i32,
    pub name: String,
    pub resource_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Person {
    pub resource_url: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct AverageRating {
    pub average: f32,
    pub count: i32,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub title: String,
    #[serde(rename = "type_")]
    pub track_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Video {
    pub description: String,
    #[serde(rename = "duration")]
    pub duration_in_seconds: i32,
    pub embed: bool,
    pub title: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub title: String,
    pub id: i32,
    pub artists: Vec<Artist>,
    pub data_quality: String,
    pub thumb: String,
    pub community: Community,
    pub companies: Vec<Company>,
    pub country: String,
    pub date_added: DateTime<Local>,
    pub estimated_weight: i32,
    #[serde(rename = "extraartists")]
    pub extra_artists: Vec<Artist>,
    pub format_quantity: i32,
    pub formats: Vec<Format>,
    pub genres: Vec<String>,
    pub identifiers: Vec<Identifier>,
    pub images: Vec<Image>,
    pub labels: Vec<Label>,
    pub lowest_price: f32,
    pub master_id: i32,
    pub master_url: String,
    pub notes: String,
    pub num_for_sale: i32,
    // TODO: DateTime?
    pub released: String,
    pub released_formatted: String,
    pub resource_url: String,
    // TODO: series
    pub status: String,
    pub styles: Vec<String>,
    pub tracklist: Vec<Track>,
    pub uri: String,
    pub videos: Vec<Video>,
    pub year: i32,
}
