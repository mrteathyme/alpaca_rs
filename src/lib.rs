pub mod broker;
pub mod data;
pub mod trading;

use anyhow::Result;

#[derive(Debug)]
pub struct AlpacaError {
    code: Option<i64>,
    message: String,
    rest: std::collections::HashMap<String,serde_json::Value>
}
impl<'a> serde::Deserialize<'a> for AlpacaError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let all = serde_json::Value::deserialize(deserializer)?;
        let code = match all.get("code") {
            Some(code) => Some(code.as_i64().unwrap()),
            None => None 
        };
        let message = match all.get("message") {
            Some(message) => message.as_str().unwrap().to_string(),
            None => return Err(serde::de::Error::missing_field("message"))
        };
        let mut rest = std::collections::HashMap::new();
        for (key,value) in all.as_object().unwrap().iter() {
            if key != "code" && key != "message" {
                rest.insert(key.clone(),value.clone());
            }
        }
        Ok(Self { code, message, rest })
    }
}


impl std::error::Error for AlpacaError {}
impl std::fmt::Display for AlpacaError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let code = match self.code {
            Some(code) => code.to_string(),
            None => "N/A".to_string()
        };
        write!(f, "AlpacaError: {} ({}): {:#?}", self.message, code, self.rest)
    }
}

pub struct AlpacaRequest<T: for<'a> serde::Deserialize<'a>>(http::Request<String>,std::marker::PhantomData<T>);

impl<T: for<'a> serde::Deserialize<'a>> AlpacaRequest<T> {
    fn new(req: http::Request<String>) -> Self {
        Self(req,std::marker::PhantomData)
    }
    pub async fn send<F, R, E>(self, func: F) -> anyhow::Result<T>
    where F: Fn(http::Request<String>) -> R,
        R: std::future::Future<Output = Result<bytes::Bytes, E>>,
        anyhow::Error: From<E>
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum AlpacaResponse<T> {
            Ok(T),
            Err(AlpacaError)
        }
        let response: AlpacaResponse<T> = serde_json::from_slice(&func(self.0).await?)?;
        match response {
            AlpacaResponse::Ok(data) => Ok(data),
            AlpacaResponse::Err(err) => Err(err.into())
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
    ) -> Result<http::Request<String>> {
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
    ) -> Result<AlpacaRequest<Self::Response>> {
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
    ) -> Result<http::Request<String>> {
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
    ) -> Result<AlpacaRequest<Self::Response>> {
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
    ) -> Result<http::Request<String>> {
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
    ) -> Result<AlpacaRequest<Self::Response>> {
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
    ) -> Result<http::Request<String>> {
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
    ) -> Result<AlpacaRequest<Self::Response>> {
        Ok(AlpacaRequest::new(self.as_patch_request(key, secret)?))
    }
}
