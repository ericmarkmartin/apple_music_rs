use reqwest::{Method, RequestBuilder};

pub trait Endpoint {
    fn url(&self) -> String;
    fn method() -> Method;
    fn configure(&self, builder: RequestBuilder) -> RequestBuilder;
}

pub enum Storefront {}

pub struct GetSong {
    id: String,
    storefront: String,
    include: Option<String>
}

impl GetSong {
    pub fn new(id: &str, storefront: &str, include: Option<&str>) -> GetSong {
        GetSong {
            id: id.to_owned(),
            storefront: storefront.to_owned(),
            include: include.map(|s| s.to_owned())
        }
    }

    fn get_params(&self) -> Vec<(&str, &str)> {
        let mut params = Vec::with_capacity(2);
        params.push(("id", self.id.as_str()));
        if let Some(ref i) = self.include { params.push(("include", i)) }
        params
    }
}

impl Endpoint for GetSong {
    fn url(&self) -> String {
        format!("catalog/{}/songs", self.storefront)
    }

    fn method() -> Method { Method::GET }

    fn configure(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.query(&self.get_params())
    }
}

impl super::Client {
    pub fn get_song(
        &self,
        id: &str,
        storefront: &str,
        include: Option<&str>
    ) -> reqwest::Result<reqwest::Response> {
        self.request(&GetSong::new(id, storefront, include))
    }
}
