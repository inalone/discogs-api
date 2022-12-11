use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub height: i32,
    pub width: i32,
    pub resource_url: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub uri: String,
    pub uri150: String,
}
