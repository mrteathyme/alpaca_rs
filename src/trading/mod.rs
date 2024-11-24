const DOMAIN: &str = "https://api.alpaca.markets";
pub mod orders;
pub mod accounts;


#[derive(serde::Serialize, serde::Deserialize, Copy, Clone, Debug)]
pub enum AssetClass {
    #[serde(rename = "us_equity")]
    USEquity,
    #[serde(rename = "us_option")]
    UsOption,
    #[serde(rename = "crypto")]
    Crypto,
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

    pub mod get_all_assets {
        #[derive(Clone, Copy, Debug, serde::Serialize)]
        pub struct Request;
        impl crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/assets";
            type Response = Vec<super::Asset>;
            fn uri(&self) -> String {
                format!("{}/{}", Self::DOMAIN, Self::ENDPOINT)
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
            fn uri(&self) -> String {
                format!("{}/{}/{}", Self::DOMAIN, Self::ENDPOINT, self.id)
            }
        }
    }
}
