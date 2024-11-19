const DOMAIN: &'static str = "https://data.alpaca.markets";

pub mod crypto {
    use crate::{IntoGetRequest, AlpacaRequest};
    #[derive(serde::Deserialize, Debug, Clone)]
    pub struct Bar {
        #[serde(rename = "t")]
        timestamp: chrono::DateTime<chrono::Utc>,
        #[serde(rename = "o")]
        open: f64,
        #[serde(rename = "h")]
        high: f64,
        #[serde(rename = "l")]
        low: f64,
        #[serde(rename = "c")]
        close: f64,
        #[serde(rename = "v")]
        volume: f64,
        #[serde(rename = "vw")]
        vwap: f64,
        #[serde(rename = "n")]
        number_of_trades: i64,
    }
    impl Bar {
        pub fn new(timestamp: chrono::DateTime<chrono::Utc>, open: f64, high: f64, low: f64, close: f64, volume: f64, vwap: f64, number_of_trades: i64) -> Self {
            Self {
                timestamp,
                open,
                high,
                low,
                close,
                volume,
                vwap,
                number_of_trades,
            }
        }
        pub fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
            self.timestamp
        }
        pub fn open(&self) -> f64 {
            self.open
        }
        pub fn high(&self) -> f64 {
            self.high
        }
        pub fn low(&self) -> f64 {
            self.low
        }
        pub fn close(&self) -> f64 {
            self.close
        }
        pub fn volume(&self) -> f64 {
            self.volume
        }
        pub fn vwap(&self) -> f64 {
            self.vwap
        }
        pub fn number_of_trades(&self) -> i64 {
            self.number_of_trades
        }
    }

    #[derive(serde::Deserialize, Debug, Clone)]
    pub struct HistoricalBars {
        bars: std::collections::HashMap<String,Vec<Bar>>,
        next_page_token: Option<String>,
    }
    impl HistoricalBars {
        pub fn bars(&self) -> &std::collections::HashMap<String,Vec<Bar>> {
            &self.bars
        }
        pub fn next_page_token(&self) -> Option<&String> {
            self.next_page_token.as_ref()
        }
    }

    #[derive(Debug, Clone)]
    pub enum Timeframe {
        Minute(u8),
        Hour(u8),
        Day,
        Week,
        Month(u8),
    }
   
    impl serde::Serialize for Timeframe {
          fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
          where
              S: serde::Serializer,
          {
              let timeframe_str: String = self.clone().try_into().map_err(serde::ser::Error::custom)?;
              serializer.serialize_str(&timeframe_str)
          }
    }


    impl TryInto<String> for Timeframe {
    type Error = &'static str;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Timeframe::Minute(n) if (1..=59).contains(&n) => Ok(format!("{}Min", n)),
            Timeframe::Hour(n) if (1..=23).contains(&n) => Ok(format!("{}H", n)),
            Timeframe::Day => Ok("1D".to_string()),
            Timeframe::Week => Ok("1W".to_string()),
            Timeframe::Month(n) if [1, 2, 3, 4, 6, 12].contains(&n) => Ok(format!("{}M", n)),
            _ => Err("Invalid timeframe value"),
        }
    }
}

    #[derive(serde::Serialize)]
    pub enum SortDirection {
        #[serde(rename = "asc")]
        Ascending,
        #[serde(rename = "desc")]
        Descending,
    }

    impl std::fmt::Display for SortDirection {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                SortDirection::Ascending => write!(f, "asc"),
                SortDirection::Descending => write!(f, "desc"),
            }
        }
    }

    pub fn historical_bars(key: &str, secret: &str, symbols: Vec<&str>, timeframe: Timeframe, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>, limit: Option<i64>, page_token: Option<&str>, sort: Option<SortDirection>) -> anyhow::Result<crate::AlpacaRequest<HistoricalBars>> {
        #[derive(serde::Serialize)]
        pub struct Request<'a> {
            symbols: Vec<&'a str>,
            start: Option<chrono::DateTime<chrono::Utc>>,
            end: Option<chrono::DateTime<chrono::Utc>>,
            timeframe: Timeframe,
            limit: Option<i64>,
            page_token: Option<&'a str>,
            sort: Option<SortDirection>,
        }

        impl<'a> crate::IntoGetRequest for Request<'a> {
            const DOMAIN: &'static str = crate::data::DOMAIN;
            const ENDPOINT: &'static str = "/v1beta3/crypto/us/bars";
            type Response = HistoricalBars;
            fn uri(&self) -> String {
                let mut uri = format!("{}{}", Self::DOMAIN, Self::ENDPOINT);
                uri.push_str("?");
                uri.push_str("symbols=");
                uri.push_str(&self.symbols.iter().fold(String::new(), |acc, symbol| {
                    format!("{}{},", acc, symbol)
                }));
                if let Some(start) = self.start {
                    uri.push_str(&format!("&start={}", start.timestamp_millis()));
                }
                if let Some(end) = self.end {
                    uri.push_str(&format!("&end={}", end.timestamp_millis()));
                }
                uri.push_str(&format!("&timeframe={}", <Timeframe as TryInto<String>>::try_into(self.timeframe.clone()).unwrap())); //safe to unwrap because we input validate
                if let Some(limit) = self.limit {
                    uri.push_str(&format!("&limit={}", limit));
                }
                if let Some(page_token) = self.page_token {
                    uri.push_str(&format!("&page_token={}", page_token));
                }
                if let Some(sort) = &self.sort {
                    uri.push_str(&format!("&sort={}", sort));
                }
                uri
            }
        }

        Ok(Request {
            symbols,
            start,
            end,
            timeframe,
            limit,
            page_token,
            sort,
        }.as_request(key, secret)?)
    }


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
            fn uri(&self) -> String {
                let mut uri = format!("{}{}", Self::DOMAIN, Self::ENDPOINT);
                if self.symbols.is_empty() { return uri; }
                uri.push_str("?");
                uri.push_str(&self.symbols.iter().fold(String::new(), |acc, symbol| {
                    format!("{}{}&", acc, symbol)
                }));
                uri
            }
            /*
            fn params_string(&self) -> String {
                let mut params = self.symbols.iter().fold(String::new(), |acc, symbol| {
                    format!("{}{},", acc, symbol)
                });
                if params.ends_with(",") {
                    params.pop();
                }
                format!("?symbols={}", params)
            }
            */
        }
    }
}
