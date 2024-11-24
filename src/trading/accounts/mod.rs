use crate::{AlpacaRequest, IntoGetRequest};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
pub enum AccountStatus { //ToDo: Serde rename these takes like 20 seconds im just lazy af
    ONBOARDING,
    SUBMISSION_FAILED,
    SUBMITTED,
    ACCOUNT_UPDATED,
    APPROVAL_PENDING,
    ACTIVE,
    REJECTED
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub account_number: Option<String>,
    pub status: AccountStatus,
    pub currency: Option<String>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub cash: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub portfolio_value: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub non_marginable_buying_power: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub accrued_fees: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub pending_transfer_in: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub pending_transfer_out: Option<f64>,
    pub pattern_day_trader: Option<bool>,
    pub trade_suspended_by_user: Option<bool>,
    pub trading_blocked: Option<bool>,
    pub transfers_blocked: Option<bool>,
    pub account_blocked: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub shorting_enabled: Option<bool>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub long_market_value: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub short_market_value: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub equity: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub last_equity: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub multiplier: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub buying_power: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub initial_margin: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub maintenance_margin: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub sma: Option<f64>,
    pub daytrade_count: Option<i32>,
    pub balance_asof: Option<String>, //ToDo: Figure out a way to parse this to chrono::DateTime given its missing some format data
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub last_maintenance_margin: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub daytrading_buying_power: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub regt_buying_power: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub options_buying_power: Option<f64>,
    pub options_approved_level: Option<i32>,
    pub options_trading_level: Option<i32>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub intraday_adjustments: Option<f64>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub pending_reg_taf_fees: Option<f64>,
}


pub fn get_account(key: &str, secret: &str) -> anyhow::Result<AlpacaRequest<Account>> {
        #[derive(Serialize)]
        pub struct Request;

        impl<'a> crate::IntoGetRequest for Request {
            const DOMAIN: &'static str = crate::trading::DOMAIN;
            const ENDPOINT: &'static str = "/v2/account";
            type Response = Account;
            fn uri(&self) -> String {
                format!("{}{}", Self::DOMAIN, Self::ENDPOINT)
            }
        }

        Ok(Request.as_request(key, secret)?)
    }
