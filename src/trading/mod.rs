const DOMAIN: &str = "https://api.alpaca.markets/v2";

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::Buy => "buy",
                Side::Sell => "sell",
            }
        )
    }
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

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderType::Market => "market",
                OrderType::Limit => "limit",
                OrderType::Stop => "stop",
                OrderType::StopLimit => "stop_limit",
                OrderType::TrailingStop => "trailing_stop",
            }
        )
    }
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

impl std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeInForce::GTC => "gtc",
                TimeInForce::IOC => "ioc",
                TimeInForce::FOK => "fok",
                TimeInForce::OPG => "opg",
                TimeInForce::CLS => "cls",
                TimeInForce::DAY => "day",
            }
        )
    }
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

impl std::fmt::Display for OrderClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderClass::Simple => "simple",
                OrderClass::Bracket => "bracket",
                OrderClass::OCO => "oco",
                OrderClass::OTO => "oto",
            }
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub struct TakeProfit {}

impl std::fmt::Display for TakeProfit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
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

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OrderStatus::New => "new",
                OrderStatus::PartiallyFilled => "partially_filled",
                OrderStatus::Filled => "filled",
                OrderStatus::DoneForDay => "done_for_day",
                OrderStatus::Canceled => "canceled",
                OrderStatus::Expired => "expired",
                OrderStatus::Replaced => "replaced",
                OrderStatus::PendingCancel => "pending_cancel",
                OrderStatus::PendingReplace => "pending_replace",
                OrderStatus::Accepted => "accepted",
                OrderStatus::PendingNew => "pending_new",
                OrderStatus::AcceptedForBidding => "accepted_for_bidding",
                OrderStatus::Stopped => "stopped",
                OrderStatus::Rejected => "rejected",
                OrderStatus::Suspended => "suspended",
                OrderStatus::Calculated => "calculated",
            }
        )
    }
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

impl std::fmt::Display for PositionIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PositionIntent::BuyToOpen => "buy_to_open",
                PositionIntent::BuyToClose => "buy_to_close",
                PositionIntent::SellToOpen => "sell_to_open",
                PositionIntent::SellToClose => "sell_to_close",
            }
        )
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

impl std::fmt::Display for AssetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetClass::USEquity => "us_equity",
                AssetClass::UsOption => "us_option",
                AssetClass::Crypto => "crypto",
            }
        )
    }
}

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

pub mod assets {
    #[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
    pub enum AssetStatus {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive
    }
    impl std::fmt::Display for AssetStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    AssetStatus::Active => "active",
                    AssetStatus::Inactive => "inactive",
                }
            )
        }
    }

    use serde_with::rust::deserialize_ignore_any;
    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    pub enum Atrributes {
        #[serde(rename = "ptp_no_exception")]
        PtpNoException,
        #[serde(rename = "ptp_with_exception")]
        PtpWithException,
        #[serde(rename = "ipo")]
        Ipo,
        #[serde(rename = "has_options")]
        HasOptions,
        #[serde(rename = "options_late_close")]
        OptionsLateClose,
        #[serde(rename = "fractional_eh_enabled")]
        FractionalEhEnabled,
    }
    impl std::fmt::Display for Atrributes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Atrributes::PtpNoException => "ptp_no_exception",
                    Atrributes::PtpWithException => "ptp_with_exception",
                    Atrributes::Ipo => "ipo",
                    Atrributes::HasOptions => "has_options",
                    Atrributes::OptionsLateClose => "options_late_close",
                    Atrributes::FractionalEhEnabled => "fractional_eh_enabled",
                }
            )
        }
    }

    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    pub enum Exchange {
        #[serde(rename = "AMEX")]
        Amex,
        #[serde(rename = "ARCA")]
        Arca,
        #[serde(rename = "BATS")]
        Bats,
        #[serde(rename = "NYSE")]
        Nyse,
        #[serde(rename = "NASDAQ")]
        Nasdaq,
        #[serde(rename = "NYSEARCA")]
        Nysearca,
        #[serde(rename = "OTC")]
        Otc,
    }
    impl std::fmt::Display for Exchange {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
            f,
            "{}",
            match self {
                Exchange::Amex => "AMEX",
                Exchange::Arca => "ARCA",
                Exchange::Bats => "BATS",
                Exchange::Nyse => "NYSE",
                Exchange::Nasdaq => "NASDAQ",
                Exchange::Nysearca => "NYSEARCA",
                Exchange::Otc => "OTC",
            }
            )
        }
    }
           

    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    pub struct Asset {
        pub id : String,
        pub class : super::AssetClass,
        pub exchange : String,
        pub symbol : String,
        pub name : String,
        pub status : String,
        pub tradable : bool,
        pub marginable : bool,
        pub shortable : bool,
        pub easy_to_borrow : bool,
        pub fractionable : bool,
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub margin_requirement_long : Option<f64>,
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub margin_requirement_short : Option<f64>,
        pub attributes : Option<Vec<Atrributes>>,
    }
    impl std::fmt::Display for Asset {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "Asset: {{\n\tid: {}\n\tclass: {}\n\texchange: {}\n\tsymbol: {}\n\tname: {}\n\tstatus: {}\n\ttradable: {}\n\tmarginable: {}\n\tshortable: {}\n\teasy_to_borrow: {}\n\tfractionable: {}\n\tmargin_requirement_long: {}\n\tmargin_requirement_short: {}\n\tattributes: {:?}\n}}",
                self.id,
                self.class,
                self.exchange,
                self.symbol,
                self.name,
                self.status,
                self.tradable,
                self.marginable,
                self.shortable,
                self.easy_to_borrow,
                self.fractionable,
                self.margin_requirement_long.unwrap_or(0.0),
                self.margin_requirement_short.unwrap_or(0.0),
                self.attributes
            )
        }
    }

    pub mod get_all_assets {
        #[derive(Clone, Copy, Debug, serde::Serialize)]
        pub struct Request;
        impl crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/assets";
            type Response = Vec<super::Asset>;
            fn params_string(&self) -> String {
                let params = String::new();
                if !params.is_empty() {
                    format!("?{}", params)
                } else {
                    params
                }
            }
        }
    }

    pub mod get_asset {
        #[derive(Clone, Copy, Debug, serde::Serialize)]
        pub struct Request<'a> {
            id: &'a str,
        }
        impl<'a> Request<'a> {
            pub fn new(id: &'a str) -> Self {
                Self { id }
            }
        }
        impl crate::IntoGetRequest for Request<'_> {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/assets";
            type Response = super::Asset;
            fn params_string(&self) -> String {
                format!("/{}", self.id)
            }
        }
    }
}

pub mod orders {
    #[serde_with::serde_as]
    #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
    pub struct Order {
        pub id: Option<uuid::Uuid>,
        pub client_order_id: Option<uuid::Uuid>,
        pub created_at: Option<chrono::DateTime<chrono::Utc>>,
        pub updates_at: Option<chrono::DateTime<chrono::Utc>>,
        pub filled_at: Option<chrono::DateTime<chrono::Utc>>,
        pub expired_at: Option<chrono::DateTime<chrono::Utc>>,
        pub canceled_at: Option<chrono::DateTime<chrono::Utc>>,
        pub failed_at: Option<chrono::DateTime<chrono::Utc>>,
        pub replaced_at: Option<chrono::DateTime<chrono::Utc>>,
        pub replaced_by: Option<String>,
        pub replaces: Option<String>,
        pub asset_id: String,
        pub symbol: String,
        pub asset_class: Option<super::AssetClass>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub notional: Option<f64>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub qty: Option<f64>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub filled_qty: Option<f64>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub filled_avg_price: Option<f64>,
        pub order_class: super::OrderClass,
        #[serde(rename = "type")]
        pub order_type: super::OrderType,
        pub side: super::Side,
        pub time_in_force: super::TimeInForce,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub limit_price: Option<f64>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub stop_price: Option<f64>,
        pub status: Option<super::OrderStatus>,
        pub extended_hours: Option<super::ExtendedHours>,
        pub legs: Option<Vec<Order>>,
        
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub trail_percent: Option<f64>,
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub trail_price: Option<f64>,
        #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
        pub hwm: Option<f64>,
        pub position_intent: Option<super::PositionIntent>,
    }

    pub mod create_order {
        #[derive(Clone, Debug)]
        pub struct Response(super::Order);
        impl std::ops::Deref for Response {
            type Target = super::Order;
            fn deref(&self) -> &Self::Target {
            &self.0
            }
        }

        impl<'de> serde::Deserialize<'de> for Response {
            fn deserialize<D>(deserializer: D) -> Result<Response, D::Error>
            where
            D: serde::Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                match (value.get("code"), value.get("message")) {
                    (Some(code), Some(message)) => {
                        Err(serde::de::Error::custom(format!("{}: {}", code, message)))
                    }
                    _ => {
                        let order = match super::Order::deserialize(value) {
                            Ok(order) => order,
                            Err(e) => return Err(serde::de::Error::custom(e)),
                        };
                        Ok(Response(order))
                    }
                }
            }
        }

        #[serde_with::serde_as]
        #[derive(serde::Serialize, Copy, Clone, Debug)]
        pub struct Request<'a> {
            symbol: &'a str,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            qty: Option<f64>,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            notional: Option<f64>,
            side: super::super::Side,
            #[serde(rename = "type")]
            _type: super::super::OrderType,
            time_in_force: super::super::TimeInForce,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            limit_price: Option<f64>,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            stop_price: Option<f64>,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            trail_price: Option<f64>,
            #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
            trail_percent: Option<f64>,
            extended_hours: Option<super::super::ExtendedHours>,
            client_order_id: Option<uuid::Uuid>,
        }
        impl<'a> Request<'a> {
            pub fn new(
                symbol: &'a str,
                side: super::super::Side,
                _type: super::super::OrderType,
                time_in_force: super::super::TimeInForce,
                extended_hours: Option<super::super::ExtendedHours>,
                client_order_id: Option<uuid::Uuid>,
                limit_price: Option<f64>,
                stop_price: Option<f64>,
                trail_price: Option<f64>,
                trail_percent: Option<f64>,
                qty: Option<f64>,
                notional: Option<f64>,
            ) -> Result<Self, Box<dyn std::error::Error>> {
                match (qty,notional) {
                    (Some(_), Some(_)) => return Err("Cannot have both qty and notional".into()),
                    (None, None) => return Err("Must have either qty or notional".into()),
                    _ => (),
                }
                Ok(Self {
                    symbol,
                    qty,
                    notional,
                    side,
                    _type,
                    time_in_force,
                    limit_price,
                    stop_price,
                    trail_price,
                    trail_percent,
                    extended_hours,
                    client_order_id,
                })
            }
        }
        impl crate::IntoPostRequest for Request<'_> {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            type Response = Response;
            const ENDPOINT: &'static str = "/orders";
        }
    }
    pub mod get_orders {
        #[serde_with::serde_as]
        #[derive(serde::Serialize, Clone, Debug)]
        pub struct Request<'a> {
            status: Option<&'a str>,
            limit: Option<u32>,
            after: Option<&'a str>,
            until: Option<&'a str>,
            direction: Option<&'a str>,
            nested: Option<bool>,
            symbols: Option<String>,
            side: Option<super::super::Side>,
        }
        impl<'a> Request<'a> {
            pub fn new(
                status: Option<&'a str>,
                limit: Option<u32>,
                after: Option<&'a str>,
                until: Option<&'a str>,
                direction: Option<&'a str>,
                nested: Option<bool>,
                symbols: Option<Vec<&str>>,
                side: Option<super::super::Side>,
            ) -> Self {
                let mut symbols_str = String::new();
                if let Some(symbols) = symbols {
                    symbols_str = symbols
                        .iter()
                        .fold(symbols_str, |acc, symbol| format!("{},{}", acc, symbol));
                }
                let symbols = if symbols_str.is_empty() {
                    None
                } else {
                    Some(symbols_str)
                };
                Self {
                    status,
                    limit,
                    after,
                    until,
                    direction,
                    nested,
                    symbols,
                    side,
                }
            }
        }

        impl crate::IntoGetRequest for Request<'_> {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/orders";
            type Response = Vec<super::Order>;
            fn params_string(&self) -> String {
                let mut params = String::new();
                if let Some(status) = self.status {
                    params.push_str(&format!("status={}&", status));
                }
                if let Some(limit) = self.limit {
                    params.push_str(&format!("limit={}&", limit));
                }
                if let Some(after) = self.after {
                    params.push_str(&format!("after={}&", after));
                }
                if let Some(until) = self.until {
                    params.push_str(&format!("until={}&", until));
                }
                if let Some(direction) = self.direction {
                    params.push_str(&format!("direction={}&", direction));
                }
                if let Some(nested) = self.nested {
                    params.push_str(&format!("nested={}&", nested));
                }
                if let Some(symbols) = &self.symbols {
                    params.push_str(&format!("symbols={}&", symbols));
                }
                if let Some(side) = self.side {
                    params.push_str(&format!("side={}&", side));
                }
                if params.ends_with('&') {
                    params.pop();
                }
                if !params.is_empty() {
                    format!("?{}", params)
                } else {
                    params
                }
            }
        }
    }
}
