use reqwest::header::USER_AGENT;
use reqwest::{Client, Error, RequestBuilder, Response};

use crate::Artist;

pub const API_URL: &'static str = "https://api.discogs.com";

pub struct Discogs {
    pub api_endpoint: String,
    pub user_agent: String,
    http_client: Client,
}

impl Discogs {
    // TODO: add rate limit
    pub async fn make_request(&self, url: &str) -> Result<Response, Error> {
        let api_call: RequestBuilder = self
            .http_client
            .get(url)
            .header(USER_AGENT, self.user_agent.as_str());
        api_call.send().await
    }

    pub fn new(user_agent: &str) -> Self {
        Discogs {
            api_endpoint: API_URL.to_string(),
            user_agent: user_agent.to_string(),
            http_client: Client::new(),
        }
    }
}
