#![feature(unsized_const_params)]
pub mod broker;
pub mod data;
pub mod trading;

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum AlpacaResponse<T> {
    Ok(T),
    Err(serde_json::Value)
}

pub struct AlpacaRequest<T: for<'a> serde::Deserialize<'a>>(http::Request<String>,std::marker::PhantomData<T>);

impl<T: for<'a> serde::Deserialize<'a>> AlpacaRequest<T> {
    fn new(req: http::Request<String>) -> Self {
        Self(req,std::marker::PhantomData)
    }
    pub async fn send<F, R>(self, func: F) -> Result<T, Box<dyn std::error::Error>>
    where F: Fn(http::Request<String>) -> R,
        R: std::future::Future<Output = Result<bytes::Bytes, Box<dyn std::error::Error>>>,
    { 
        let response: AlpacaResponse<T> = serde_json::from_slice(&func(self.0).await?)?;
        match response {
            AlpacaResponse::Ok(data) => Ok(data),
            AlpacaResponse::Err(err) => Err(serde_json::to_string(&err)?.into())
        }
    }
}

trait IntoPostRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn uri(&self) -> String;
    fn as_post_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        Ok(http::request::Builder::new()
            .method("POST")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(self.uri())
            .body(serde_json::to_string(self)?)?)
    }
    fn as_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<AlpacaRequest<Self::Response>, Box<dyn std::error::Error>> {
        Ok(AlpacaRequest::new(self.as_post_request(key, secret)?))
    }
}

trait IntoGetRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn uri(&self) -> String;
    fn as_get_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        Ok(http::request::Builder::new()
            .method("GET")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(self.uri())
            .body(String::new())?)
    }
    fn as_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<AlpacaRequest<Self::Response>, Box<dyn std::error::Error>> {
        Ok(AlpacaRequest::new(self.as_get_request(key, secret)?))
    }
}

trait IntoDeleteRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn uri(&self) -> String;
    fn as_delete_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        Ok(http::request::Builder::new()
            .method("DELETE")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(self.uri())
            .body(String::new())?)
    }
    fn as_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<AlpacaRequest<Self::Response>, Box<dyn std::error::Error>> {
        Ok(AlpacaRequest::new(self.as_delete_request(key, secret)?))
    }
}

trait IntoPatchRequest: serde::Serialize {
    const DOMAIN: &'static str;
    const ENDPOINT: &'static str;
    type Response: for<'a> serde::Deserialize<'a>;
    fn uri(&self) -> String;
    fn as_patch_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        Ok(http::request::Builder::new()
            .method("PATCH")
            .header("APCA-API-KEY-ID", key)
            .header("APCA-API-SECRET-KEY", secret)
            .uri(self.uri())
            .body(serde_json::to_string(self)?)?)
    }
    fn as_request(
        &self,
        key: &str,
        secret: &str,
    ) -> Result<AlpacaRequest<Self::Response>, Box<dyn std::error::Error>> {
        Ok(AlpacaRequest::new(self.as_patch_request(key, secret)?))
    }
}
