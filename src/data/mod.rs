const DOMAIN: &'static str = "https://data.alpaca.markets";

pub mod crypto {
    pub mod latest_orderbook {
        #[derive(serde::Serialize, Clone)]
        pub struct Request<'a> {
            pub symbols: Vec<&'a str>,
        }
        impl<'a> Request<'a> {
            pub fn new(symbols: Vec<&'a str>) -> Self {
                Self {
                    symbols,
                }
            }
        }
       
        #[derive(serde::Deserialize, Debug, Clone)]
        pub struct OrderPage {
            #[serde(rename = "p")]
            price: f64,
            #[serde(rename = "s")]
            size: f64,
        }

        #[derive(serde::Deserialize, Debug, Clone)]
        pub struct OrderBook {
            #[serde(rename = "t")]
            timestamp: chrono::DateTime<chrono::Utc>,
            #[serde(rename = "b")]
            bids: Vec<OrderPage>,
            #[serde(rename = "a")]
            asks: Vec<OrderPage>,
        }
       
        #[derive(serde::Deserialize, Debug, Clone)]
        pub struct Response {
            orderbooks: std::collections::HashMap<String, OrderBook>
        }       
        impl std::ops::Deref for Response {
            type Target = std::collections::HashMap<String, OrderBook>;
            fn deref(&self) -> &Self::Target {
                &self.orderbooks
            }
        }

        impl crate::IntoGetRequest for Request<'_> {
            const DOMAIN: &'static str = crate::data::DOMAIN;
            const ENDPOINT: &'static str = "/v1beta3/crypto/us/latest/orderbooks";
            type Response = Response;
            fn params_string(&self) -> String {
                let mut params = self.symbols.iter().fold(String::new(), |acc, symbol| {
                    format!("{}{},", acc, symbol)
                });
                if params.ends_with(",") {
                    params.pop();
                }
                format!("?symbols={}", params)
            }
        }
    }
}
