use crate::{IntoGetRequest, IntoPostRequest, IntoPatchRequest, IntoDeleteRequest};
#[derive(Copy, Clone, Debug)]
pub enum ExtendedHours {
    True,
    False
}

impl serde::Serialize for ExtendedHours {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ExtendedHours::True => serializer.serialize_bool(true),
            ExtendedHours::False => serializer.serialize_bool(false),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExtendedHours {
    fn deserialize<D>(deserializer: D) -> Result<ExtendedHours, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b = bool::deserialize(deserializer)?;
        match b {
            true => Ok(ExtendedHours::True),
            false => Ok(ExtendedHours::False),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum AssetClass {
    #[serde(rename = "us_equity")]
    USEquity,
    #[serde(rename = "us_option")]
    UsOption,
    #[serde(rename = "crypto")]
    Crypto,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum PositionIntent {
    #[serde(rename = "buy_to_open")]
    BuyToOpen,
    #[serde(rename = "buy_to_close")]
    BuyToClose,
    #[serde(rename = "sell_to_open")]
    SellToOpen,
    #[serde(rename = "sell_to_close")]
    SellToClose,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum OrderStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "done_for_day")]
    DoneForDay,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "replaced")]
    Replaced,
    #[serde(rename = "pending_cancel")]
    PendingCancel,
    #[serde(rename = "pending_replace")]
    PendingReplace,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "pending_new")]
    PendingNew,
    #[serde(rename = "accepted_for_bidding")]
    AcceptedForBidding,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "calculated")]
    Calculated,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum OrderClass {
    #[serde(rename = "simple")]
    #[serde(alias = "")]
    Simple,
    #[serde(rename = "bracket")]
    Bracket,
    #[serde(rename = "oco")]
    OCO,
    #[serde(rename = "oto")]
    OTO,
}


#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    GTC,
    #[serde(rename = "ioc")]
    IOC,
    #[serde(rename = "fok")]
    FOK,
    #[serde(rename = "opg")]
    OPG,
    #[serde(rename = "cls")]
    CLS,
    #[serde(rename = "day")]
    DAY,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "trailing_stop")]
    TrailingStop,
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[serde_with::serde_as]
#[derive(derive_builder::Builder,serde::Serialize, serde::Deserialize, Clone, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Order {
    #[builder(setter(into, strip_option))]
    pub id: Option<uuid::Uuid>,

    #[builder(setter(into, strip_option))]
    pub client_order_id: Option<uuid::Uuid>,
    #[builder(setter(skip))]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub updates_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub filled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub expired_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub failed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub replaced_at: Option<chrono::DateTime<chrono::Utc>>,
    #[builder(setter(skip))]
    pub replaced_by: Option<String>,
    #[builder(setter(into, strip_option))]
    pub replaces: Option<String>,
    #[builder(setter(skip))]
    pub asset_id: Option<String>,
    #[builder(setter(into))]
    pub symbol: String,
    #[builder(setter(into, strip_option))]
    pub asset_class: Option<AssetClass>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub notional: Option<f64>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub qty: Option<f64>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(skip))]
    pub filled_qty: Option<f64>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(skip))]
    pub filled_avg_price: Option<f64>,
    #[builder(setter(into, strip_option))]
    pub order_class: Option<OrderClass>,
    #[serde(rename = "type")]
    #[builder(setter(into, strip_option))]
    pub order_type: OrderType,
    pub side: Side,
    pub time_in_force: TimeInForce,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub limit_price: Option<f64>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub stop_price: Option<f64>,
    #[builder(setter(into, strip_option))]
    pub status: Option<OrderStatus>,
    #[builder(setter(into, strip_option))]
    pub extended_hours: Option<ExtendedHours>,
    #[builder(setter(into, strip_option))]
    pub legs: Option<Vec<Order>>,
    
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub trail_percent: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub trail_price: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    #[builder(setter(into, strip_option))]
    pub hwm: Option<f64>,
    #[builder(setter(into, strip_option))]
    pub position_intent: Option<PositionIntent>,
}

impl OrderBuilder {
    fn validate(&self) -> Result<(), String> {
        if self.qty.is_none() && self.notional.is_none() {
            return Err("qty or notional is required".into());
        }
        if self.qty.is_some() && self.notional.is_some() {
            return Err("qty and notional are mutually exclusive".into());
        }
        if self.symbol.is_none() {
            return Err("symbol is required".into());
        }
        if self.side.is_none() {
            return Err("side is required".into());
        }
        if self.order_type.is_none() {
            return Err("order_type is required".into());
        }
        if self.time_in_force.is_none() {
            return Err("time_in_force is required".into());
        }
        if let (Some(OrderType::Limit), None) = (self.order_type, self.limit_price) {
            return Err("limit_price is required for limit orders".into());
        }
        if let (Some(OrderType::Stop), None) = (self.order_type, self.stop_price) {
            return Err("stop_price is required for stop orders".into());
        }
        if let (Some(OrderType::StopLimit), None) = (self.order_type, self.stop_price) {
            return Err("stop_price is required for stop limit orders".into());
        }
        if let (Some(OrderType::StopLimit), None) = (self.order_type, self.limit_price) {
            return Err("limit_price is required for stop limit orders".into());
        }
        if let (Some(OrderType::TrailingStop), None) = (self.order_type, self.trail_price) {
            return Err("trail_price is required for trailing stop orders".into());
        }
        if let (Some(OrderType::TrailingStop), None) = (self.order_type, self.trail_percent) {
            return Err("trail_percent is required for trailing stop orders".into());
        }
        Ok(())
    }
}

impl Order {
    pub fn get_order(key: &str, secret: &str, id: uuid::Uuid) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request {
            id: uuid::Uuid,
        }
        impl crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/orders";
            type Response = Order;
            fn uri(&self) -> String {
                format!("{}{}/{}", Self::DOMAIN, Self::ENDPOINT, self.id)
            }
        }
        Request { id }.as_get_request(key, secret)
    }
    pub fn get_orders(key: &str, secret: &str, ) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request;
        impl crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/orders";
            type Response = Vec<Order>;
            fn uri(&self) -> String {
                format!("{}{}", Self::DOMAIN, Self::ENDPOINT)
            }
        }
        Request.as_get_request(key, secret)
    }
    pub fn create_order(&self, key: &str, secret: &str) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request {
            symbol: String,
            side: Side,
            #[serde(rename = "type")]
            order_type: OrderType,
            qty: Option<f64>,
            notional: Option<f64>,
            time_in_force: TimeInForce,
            limit_price: Option<f64>,
            stop_price: Option<f64>,
            trail_price: Option<f64>,
            trail_percent: Option<f64>,
            extended_hours: Option<ExtendedHours>,
            client_order_id: Option<uuid::Uuid>,
            order_class: Option<OrderClass>,
            position_intent: Option<PositionIntent>
        }
        impl crate::IntoPostRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/orders";
            type Response = Order;
            fn uri(&self) -> String {
                format!("{}{}", Self::DOMAIN, Self::ENDPOINT)
            }
        }
        Request {
            symbol: self.symbol.clone(),
            side: self.side,
            order_type: self.order_type,
            qty: self.qty,
            notional: self.notional,
            time_in_force: self.time_in_force,
            limit_price: self.limit_price,
            stop_price: self.stop_price,
            trail_price: self.trail_price,
            trail_percent: self.trail_percent,
            extended_hours: self.extended_hours,
            client_order_id: self.client_order_id,
            order_class: self.order_class,
            position_intent: self.position_intent
        }.as_post_request(key, secret)
    }
    pub fn replace_order(&self, key: &str, secret: &str, id: uuid::Uuid) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request {
        id: uuid::Uuid,
        qty: Option<f64>,
        notional: Option<f64>,
        time_in_force: Option<TimeInForce>,
        limit_price: Option<f64>,
        stop_price: Option<f64>,
        trail_price: Option<f64>,
        trail_percent: Option<f64>,
        extended_hours: Option<ExtendedHours>,
        client_order_id: Option<uuid::Uuid>,
        order_class: Option<OrderClass>,
        position_intent: Option<PositionIntent>
        }
        impl crate::IntoPatchRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/orders";
            type Response = Order;
            fn uri(&self) -> String {
                format!("{}{}/{}", Self::DOMAIN, Self::ENDPOINT, self.id)
            }
        }
        Request {
        id,
        qty: self.qty,
        notional: self.notional,
        time_in_force: Some(self.time_in_force),
        limit_price: self.limit_price,
        stop_price: self.stop_price,
        trail_price: self.trail_price,
        trail_percent: self.trail_percent,
        extended_hours: self.extended_hours,
        client_order_id: self.client_order_id,
        order_class: self.order_class,
        position_intent: self.position_intent
        }.as_patch_request(key, secret)
    }
    pub fn cancel_order(key: &str, secret: &str, id: uuid::Uuid) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request {
        id: uuid::Uuid,
        }
        impl crate::IntoDeleteRequest for Request {
        const DOMAIN: &'static str = crate::trading::DOMAIN;
        const ENDPOINT: &'static str = "/v2/orders";
        type Response = Order;
            fn uri(&self) -> String {
                format!("{}{}/{}", Self::DOMAIN, Self::ENDPOINT, self.id)
            }
        }
        Request { id }.as_delete_request(key, secret)
    }
    pub fn cancel_orders(key: &str, secret: &str) -> Result<http::Request<String>, Box<dyn std::error::Error>> {
        #[derive(serde::Serialize)]
        struct Request;
        impl crate::IntoDeleteRequest for Request {
        const DOMAIN: &'static str = crate::trading::DOMAIN;
        const ENDPOINT: &'static str = "/v2/orders";
        type Response = Order;
            fn uri(&self) -> String {
                format!("{}{}", Self::DOMAIN, Self::ENDPOINT)
            }
        }
        Request.as_delete_request(key, secret)
    }
}
