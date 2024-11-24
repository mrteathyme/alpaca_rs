use crate::{AlpacaRequest, IntoGetRequest};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
pub enum PositionSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
pub struct Position {
    pub asset_id: String,
    pub symbol: String,
    pub asset_class: crate::trading::AssetClass,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub qty: f64,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub qty_available: Option<f64>,
    pub side: PositionSide,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub market_value: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub cost_basis: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub unrealized_pl: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub avg_entry_price: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub unrealized_plpc: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub current_price: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub lastday_price: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub change_today: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub unrealized_intraday_pl: f64,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub unrealized_intraday_plpc: f64,
    pub asset_marginable: bool,
}

impl Position {
    pub fn get_available_value(&self) -> Option<f64> {
        match self.qty_available {
            Some(data) => Some(self.current_price * data),
            None => None
        }
    }
}


pub fn get_all_positions(key: &str, secret: &str) -> anyhow::Result<AlpacaRequest<Vec<Position>>> {
        #[derive(Serialize)]
        pub struct Request;

        impl<'a> crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/positions";
            type Response = Vec<Position>;
            fn uri(&self) -> String {
                format!("{}{}", Self::DOMAIN, Self::ENDPOINT)
            }
        }

        Ok(Request.as_request(key, secret)?)
    }
