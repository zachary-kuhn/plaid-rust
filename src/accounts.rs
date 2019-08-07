use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Accounts {
    fn get_accounts(&self, access_token: &str) -> Result<GetAccountsResponse, Error>;
    fn get_accounts_with_options(
        &self,
        access_token: &str,
        options: Option<GetAccountsRequestOptions>,
    ) -> Result<GetAccountsResponse, Error>;
    fn get_balances(&self, access_token: &str) -> Result<GetBalancesResponse, Error>;
    fn get_balances_with_options(
        &self,
        access_token: &str,
        options: Option<GetBalancesRequestOptions>,
    ) -> Result<GetBalancesResponse, Error>;
}

#[derive(Deserialize)]
pub struct Account {
    pub account_id: String,
    pub balances: AccountBalances,
    pub mask: String,
    pub name: String,
    pub official_name: String,
    pub subtype: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub verification_status: String,
}

#[derive(Deserialize)]
pub struct AccountBalances {
    pub available: f64,
    pub current: f64,
    pub limit: f64,
    pub iso_currency_code: String,
    pub unofficial_currency_code: String,
}

#[derive(Serialize)]
struct GetAccountsRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<&'a str>>,
}

#[derive(Serialize)]
struct GetAccountsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetAccountsRequestOptions<'a>>,
}

#[derive(Deserialize)]
pub struct GetAccountsResponse {
    pub response_id: String,
    pub accounts: Vec<Account>,
}

#[derive(Serialize)]
struct GetBalancesRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetBalancesRequestOptions>,
}

#[derive(Serialize)]
struct GetBalancesRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct GetBalancesResponse {
    pub response_id: String,
    pub accounts: Vec<Account>,
}

impl<'a> Accounts for Client<'a> {
    fn get_accounts(&self, access_token: &str) -> Result<GetAccountsResponse, Error> {
        self.get_accounts_with_options(access_token, None)
    }

    fn get_accounts_with_options(
        &self,
        access_token: &str,
        options: Option<GetAccountsRequestOptions>,
    ) -> Result<GetAccountsResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetAccountsRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/accounts/get", &json_body))
    }

    fn get_balances(&self, access_token: &str) -> Result<GetBalancesResponse, Error> {
        self.get_balances_with_options(access_token, None)
    }

    fn get_balances_with_options(
        &self,
        access_token: &str,
        options: Option<GetBalancesRequestOptions>,
    ) -> Result<GetBalancesResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetBalancesRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/accounts/balance/get", &json_body))
    }
}
