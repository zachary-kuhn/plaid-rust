use crate::errors::{Error, Kind};
use crate::plaid::Client;
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
    pub official_name: Option<String>,
    pub subtype: String,
    #[serde(rename = "type")]
    pub account_type: String,
    pub verification_status: Option<String>,
}

#[derive(Deserialize)]
pub struct AccountBalances {
    pub available: Option<f64>,
    pub current: f64,
    pub limit: Option<f64>,
    pub iso_currency_code: String,
    pub unofficial_currency_code: Option<String>,
}

#[derive(Deserialize)]
pub struct ACHNumber {
    pub account: String,
    pub account_id: String,
    pub routing: String,
    pub wire_routing: String,
}

#[derive(Deserialize)]
pub struct EFTNumber {
    pub account: String,
    pub account_id: String,
    pub institution: String,
    pub branch: String,
}

#[derive(Deserialize)]
pub struct IBANNumber {
    pub account_id: String,
    pub iban: String,
    pub bic: String,
}

#[derive(Deserialize)]
pub struct BACSNumber {
    pub account: String,
    pub account_id: String,
    pub sort_code: String,
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
    pub request_id: String,
    pub accounts: Vec<Account>,
}

#[derive(Serialize)]
struct GetBalancesRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetBalancesRequestOptions<'a>>,
}

#[derive(Serialize)]
struct GetBalancesRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<&'a str>>,
}

#[derive(Deserialize)]
pub struct GetBalancesResponse {
    pub request_id: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environments::Environment;
    use crate::items::Items;
    use crate::sandbox::Sandbox;
    use std::env;

    #[test]
    fn test_get_accounts() {
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
            .create_sandbox_public_token(
                "ins_109508",
                &["auth", "identity", "income", "transactions"],
            )
            .unwrap();
        let token_resp = test_client
            .exchange_public_token(sandbox_resp.public_token.as_str())
            .unwrap();

        let accounts_resp = test_client
            .get_accounts(token_resp.access_token.as_str())
            .unwrap();

        assert_eq!(8, accounts_resp.accounts.len());

        let filtered_accounts_resp = test_client
            .get_accounts_with_options(
                token_resp.access_token.as_str(),
                Some(GetAccountsRequestOptions {
                    account_ids: Some(vec![accounts_resp.accounts[0].account_id.as_str()]),
                }),
            )
            .unwrap();

        assert_eq!(1, filtered_accounts_resp.accounts.len());
    }

    #[test]
    fn test_get_balances() {
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
            .create_sandbox_public_token(
                "ins_109508",
                &["auth", "identity", "income", "transactions"],
            )
            .unwrap();
        let token_resp = test_client
            .exchange_public_token(sandbox_resp.public_token.as_str())
            .unwrap();

        let balances_resp = test_client
            .get_balances(token_resp.access_token.as_str())
            .unwrap();

        assert_eq!(8, balances_resp.accounts.len());

        let filtered_balances_resp = test_client
            .get_balances_with_options(
                token_resp.access_token.as_str(),
                Some(GetBalancesRequestOptions {
                    account_ids: Some(vec![balances_resp.accounts[0].account_id.as_str()]),
                }),
            )
            .unwrap();

        assert_eq!(1, filtered_balances_resp.accounts.len());
    }
}
