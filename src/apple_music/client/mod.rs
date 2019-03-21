extern crate chrono;
extern crate jsonwebtoken as jwt;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod auth;
mod endpoint;

use reqwest::{Response, Result as ReqResult};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;

use endpoint::Endpoint;

const BASE_URL: &str = "https://api.music.apple.com";
const API_VERSION: &str = "v1";
const TIMEOUT_SECONDS: usize = 30;

pub struct APIConfig {
    base_url: String,
    version: String
}

impl APIConfig {
    fn url(&self) -> String {
        format!("{}/{}", self.base_url, self.version)
    }
}

impl Default for APIConfig {
    fn default() -> Self {
        APIConfig {
            base_url: BASE_URL.to_owned(),
            version: API_VERSION.to_owned()
        }
    }
}

pub struct Client {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client
}

impl Client {
    pub fn new(
        team_id: &str,
        key_id: &str,
        private_key: &str,
        api_config: APIConfig,
        connect_timeout: Duration,
        timeout: Duration
    ) -> Result<Self, ()> {
        let url = api_config.url();

        let mut headers = HeaderMap::new();
        let access_token = auth::make_token(team_id, key_id, private_key).or(Err(()))?;
        let value = HeaderValue::from_str(access_token.as_str()).or(Err(()))?;
        headers.insert("Music-User-Token", value);

        let client = reqwest::ClientBuilder::new()
            .timeout(timeout)
            .connect_timeout(connect_timeout)
            .build()
            .or(Err(()))?;

        Ok(Client {
            url,
            headers,
            client
        })
    }

    fn endpoint_url<T: Endpoint>(&self, e: &T) -> String {
        format!("{}/{}", self.url, e.url())
    }

    fn request<T: Endpoint>(&self, e: &T) -> ReqResult<Response> {
        let builder = self.client.request(T::method(), self.endpoint_url(e).as_str())
            .headers(self.headers.clone());

        e
            .configure(builder)
            .send()
    }
}
