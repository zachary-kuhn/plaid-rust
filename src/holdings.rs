use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::items::Item;
use crate::plaid::Client;
use serde::*;

trait Holdings {
    fn get_holdings(&self, access_token: &str) -> Result<GetHoldingsResponse, Error>;
}

#[derive(Deserialize)]
pub struct Security {
    pub security_id: String,
    pub cusip: String,
    pub sedol: String,
    pub isin: String,
    pub institution_security_id: String,
    pub institution_id: String,
    pub proxy_security_id: String,
    pub name: String,
    pub ticker_symbol: String,
    pub is_cash_equivalent: bool,
    #[serde(rename = "type")]
    pub security_type: String,
    pub close_price: f64,
    pub close_price_as_of: String,
    pub iso_currency_code: String,
    pub unofficial_currency_code: String,
}

#[derive(Deserialize)]
pub struct Holding {
    pub account_id: String,
    pub security_id: String,
    pub institution_value: f64,
    pub institution_price: f64,
    pub quantity: f64,
    pub institution_price_as_of: String,
    pub cost_basis: f64,
    pub iso_currency_code: String,
    pub unofficial_currency_code: String,
}

#[derive(Serialize)]
struct GetHoldingsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetHoldingsResponse {
    pub response_id: String,
    pub accounts: Vec<Account>,
    pub item: Item,
    pub securities: Vec<Security>,
    pub holdings: Vec<Holding>,
}

impl<'a> Holdings for Client<'a> {
    fn get_holdings(&self, access_token: &str) -> Result<GetHoldingsResponse, Error> {
        if access_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = GetHoldingsRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/investments/holdings/get", &json_body))
    }
}
