use crate::{AlpacaRequest, IntoGetRequest};
use serde::{Deserialize,Serialize};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum AccountStatus { //ToDo: Serde rename these takes like 20 seconds im just lazy af
    ONBOARDING,
    SUBMISSION_FAILED,
    SUBMITTED,
    ACCOUNT_UPDATED,
    APPROVAL_PENDING,
    ACTIVE,
    REJECTED
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub account_number: Option<String>,
    pub status: AccountStatus,
    pub currency: Option<String>,
    pub cash: Option<String>,
    pub portfolio_value: Option<String>,
    pub non_marginable_buying_power: Option<String>,
    pub accrued_fees: Option<String>,
    pub pending_transfer_in: Option<String>,
    pub pending_transfer_out: Option<String>,
    pub pattern_day_trader: Option<bool>,
    pub trade_suspended_by_user: Option<bool>,
    pub trading_blocked: Option<bool>,
    pub transfers_blocked: Option<bool>,
    pub account_blocked: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub shorting_enabled: Option<bool>,
    pub long_market_value: Option<String>,
    pub short_market_value: Option<String>,
    pub equity: Option<String>,
    pub last_equity: Option<String>,
    pub multiplier: Option<String>,
    pub buying_power: Option<String>,
    pub initial_margin: Option<String>,
    pub maintenance_margin: Option<String>,
    pub sma: Option<String>,
    pub daytrade_count: Option<i32>,
    pub balance_asof: Option<String>,
    pub last_maintenance_margin: Option<String>,
    pub daytrading_buying_power: Option<String>,
    pub regt_buying_power: Option<String>,
    pub options_buying_power: Option<String>,
    pub options_approved_level: Option<i32>,
    pub options_trading_level: Option<i32>,
    pub intraday_adjustments: Option<String>,
    pub pending_reg_taf_fees: Option<String>,
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
