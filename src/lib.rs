pub mod broker;
pub mod data;
pub mod trading;

#[derive(Debug)]
pub struct AlpacaError {
    code: i64,
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
            Some(code) => code.as_i64().unwrap(),
            None => return Err(serde::de::Error::missing_field("code"))
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
        write!(f, "AlpacaError: {} ({}): {:#?}", self.message, self.code, self.rest)
    }
}

pub struct AlpacaRequest<T: for<'a> serde::Deserialize<'a>>(http::Request<String>,std::marker::PhantomData<T>);

impl<T: for<'a> serde::Deserialize<'a>> AlpacaRequest<T> {
    fn new(req: http::Request<String>) -> Self {
        Self(req,std::marker::PhantomData)
    }
    pub async fn send<F, R>(self, func: F) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where F: Fn(http::Request<String>) -> R,
        R: std::future::Future<Output = Result<bytes::Bytes, Box<dyn std::error::Error + Send + Sync>>>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum AlpacaResponse<T> {
            Ok(T),
            Err(AlpacaError)
        }
        let response = serde_json::from_slice::<AlpacaResponse<T>>(&func(self.0).await?)?;
        match response {
                AlpacaResponse::Ok(data) => Ok(data),
                AlpacaResponse::Err(err) => Err(Box::new(err))
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
