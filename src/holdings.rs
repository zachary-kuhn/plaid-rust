use crate::accounts::Account;
use crate::items::Item;
use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Holdings {
    fn get_holdings(&self, access_token: &str) -> Result<GetHoldingsResponse, Error>;
}

#[derive(Deserialize)]
pub struct Security {
    security_id: String,
    cusip: String,
    sedol: String,
    isin: String,
    institution_security_id: String,
    institution_id: String,
    proxy_security_id: String,
    name: String,
    ticker_symbol: String,
    is_cash_equivalent: bool,
    #[serde(rename = "type")]
    security_type: String,
    close_price: f64,
    close_price_as_of: String,
    iso_currency_code: String,
    unofficial_currency_code: String,
}

#[derive(Deserialize)]
pub struct Holding {
    account_id: String,
    security_id: String,
    institution_value: f64,
    institution_price: f64,
    quantity: f64,
    institution_price_as_of: String,
    cost_basis: f64,
    iso_currency_code: String,
    unofficial_currency_code: String,
}

#[derive(Serialize)]
struct GetHoldingsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetHoldingsResponse {
    response_id: String,
    accounts: Vec<Account>,
    item: Item,
    securities: Vec<Security>,
    holdings: Vec<Holding>,
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
