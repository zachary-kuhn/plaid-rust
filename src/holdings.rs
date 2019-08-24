use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::items::Item;
use crate::plaid::Client;
use serde::*;

trait Holdings {
    fn get_holdings(&self, access_token: &str) -> Result<GetHoldingsResponse, Error>;
    fn get_holdings_with_options(
        &self,
        access_token: &str,
        options: Option<GetHoldingsRequestOptions>,
    ) -> Result<GetHoldingsResponse, Error>;
}

#[derive(Deserialize)]
pub struct Security {
    pub security_id: String,
    pub cusip: Option<String>,
    pub sedol: Option<String>,
    pub isin: Option<String>,
    pub institution_security_id: Option<String>,
    pub institution_id: Option<String>,
    pub proxy_security_id: Option<String>,
    pub name: Option<String>,
    pub ticker_symbol: Option<String>,
    pub is_cash_equivalent: bool,
    #[serde(rename = "type")]
    pub security_type: String,
    pub close_price: f64,
    pub close_price_as_of: Option<String>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
}

#[derive(Deserialize)]
pub struct Holding {
    pub account_id: String,
    pub security_id: String,
    pub institution_value: f64,
    pub institution_price: f64,
    pub quantity: f64,
    pub institution_price_as_of: Option<String>,
    pub cost_basis: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
}

#[derive(Serialize)]
struct GetHoldingsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetHoldingsRequestOptions<'a>>,
}

#[derive(Serialize)]
pub struct GetHoldingsRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<&'a str>>,
}

#[derive(Deserialize)]
pub struct GetHoldingsResponse {
    pub request_id: String,
    pub accounts: Vec<Account>,
    pub item: Item,
    pub securities: Vec<Security>,
    pub holdings: Vec<Holding>,
}

impl<'a> Holdings for Client<'a> {
    fn get_holdings(&self, access_token: &str) -> Result<GetHoldingsResponse, Error> {
        self.get_holdings_with_options(access_token, None)
    }

    fn get_holdings_with_options(
        &self,
        access_token: &str,
        options: Option<GetHoldingsRequestOptions>,
    ) -> Result<GetHoldingsResponse, Error> {
        if access_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = GetHoldingsRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/investments/holdings/get", &json_body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environments::Environment;
    use crate::items::Items;
    use crate::sandbox::Sandbox;
    use std::env;

    #[test]
    fn test_get_holdings() {
        let client_id = env::var("PLAID_CLIENT_ID").unwrap();
        let secret = env::var("PLAID_SECRET").unwrap();
        let public_key = env::var("PLAID_PUBLIC_KEY").unwrap();
        let test_client = Client {
            client_id: client_id.as_str(),
            secret: secret.as_str(),
            public_key: public_key.as_str(),
            environment: Environment::SANDBOX,
            http_client: reqwest::Client::new(),
        };

        let sandbox_resp = test_client
            .create_sandbox_public_token("ins_109508", &["investments"])
            .unwrap();
        let token_resp = test_client
            .exchange_public_token(sandbox_resp.public_token.as_str())
            .unwrap();

        let holdings_resp = test_client
            .get_holdings(token_resp.access_token.as_str())
            .unwrap();

        assert_eq!(8, holdings_resp.accounts.len());

        let filtered_holdings_resp = test_client
            .get_holdings_with_options(
                token_resp.access_token.as_str(),
                Some(GetHoldingsRequestOptions {
                    account_ids: Some(vec![holdings_resp.accounts[0].account_id.as_str()]),
                }),
            )
            .unwrap();

        assert_eq!(1, filtered_holdings_resp.accounts.len());
    }
}
