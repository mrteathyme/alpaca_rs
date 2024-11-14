pub mod broker;
pub mod data;
pub mod trading;

pub trait IntoPostRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn as_post_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", Self::DOMAIN, Self::ENDPOINT);
        Ok(http::request::Builder::new()
            .method("POST")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(url)
            .body(serde_json::to_string(self)?)?)
    }
    async fn send<F, R>(
        &self,
        key: &str,
        secret: &str,
        func: F,
    ) -> Result<Self::Response, Box<dyn std::error::Error>>
    where
        F: Fn(http::Request<String>) -> R,
        R: std::future::Future<Output = Result<bytes::Bytes, Box<dyn std::error::Error>>>,
    {
        let response = func(self.as_post_request(key, secret)?).await?;
        Ok(serde_json::from_slice(&response)?)
    }
}

pub trait IntoGetRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn params_string(&self) -> String;
    fn as_get_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        let mut params = self.params_string();
        if !self.params_string().is_empty() {
            params = format!("{}", params);
        }
        let url = format!("{}{}{}",Self::DOMAIN, Self::ENDPOINT, params);
        Ok(http::request::Builder::new()
            .method("GET")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(url)
            .body(String::new())?)
    }
    async fn send<F,R>(
        &self,
        key: &str,
        secret: &str,
        func: F,
    ) -> Result<Self::Response, Box<dyn std::error::Error>>
    where
        F: Fn(http::Request<String>) -> R,
        R: std::future::Future<Output = Result<bytes::Bytes, Box<dyn std::error::Error>>>,
    {
        Ok(serde_json::from_slice(&func(
            self.as_get_request(key, secret)?,
        ).await?)?)
    }
}

