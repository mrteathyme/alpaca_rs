pub mod broker;
pub mod data;
pub mod trading;


pub trait IntoPostRequest: serde::Serialize {
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
}

pub trait IntoGetRequest: serde::Serialize {
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
}

pub trait IntoDeleteRequest: serde::Serialize {
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
}

pub trait IntoPatchRequest: serde::Serialize {
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
}
