use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Accounts {
    fn get_accounts(&self, access_token: &str) -> Result<GetAccountsResponse, Error>;
    fn get_accounts_with_options(
        &self,
        access_token: &str,
        options: Option<GetAccountsRequestOptions>,
    ) -> Result<GetAccountsResponse, Error>;
}

#[derive(Deserialize)]
struct Account {
    account_id: String,
    balances: AccountBalances,
    mask: String,
    name: String,
    official_name: String,
    subtype: String,
    #[serde(rename = "type")]
    account_type: String,
    verification_status: String,
}

#[derive(Deserialize)]
struct AccountBalances {
    available: f64,
    current: f64,
    limit: f64,
    iso_currency_code: String,
    unofficial_currency_code: String,
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
    response_id: String,
    accounts: Vec<Account>,
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
}
