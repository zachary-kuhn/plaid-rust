use crate::accounts::{ACHNumber, Account, BACSNumber, EFTNumber, IBANNumber};
use crate::errors::{Error, Kind};
use crate::plaid::Client;
use serde::{Deserialize, Serialize};

pub trait Auth<'a> {
    fn get_auth_with_options(
        &self,
        access_token: &'a str,
        options: Option<GetAuthRequestOptions>,
    ) -> Result<GetAuthResponse, Error>;
    fn get_auth(&self, access_token: &'a str) -> Result<GetAuthResponse, Error>;
}

#[derive(Serialize)]
struct GetAuthRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetAuthRequestOptions<'a>>,
}

#[derive(Serialize)]
pub struct GetAuthRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<&'a str>>,
}

#[derive(Deserialize)]
pub struct AccountNumberCollection {
    pub ach: Vec<ACHNumber>,
    pub eft: Vec<EFTNumber>,
    pub international: Vec<IBANNumber>,
    pub bacs: Vec<BACSNumber>,
}

#[derive(Deserialize)]
pub struct GetAuthResponse {
    pub request_id: String,
    pub accounts: Vec<Account>,
    pub numbers: AccountNumberCollection,
}

impl<'a> Auth<'a> for Client<'a> {
    fn get_auth_with_options(
        &self,
        access_token: &str,
        options: Option<GetAuthRequestOptions>,
    ) -> Result<GetAuthResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetAuthRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/auth/get", &json_body))
    }

    fn get_auth(&self, access_token: &str) -> Result<GetAuthResponse, Error> {
        self.get_auth_with_options(access_token, None)
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
    fn test_get_auth() {
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

        let auth_resp = test_client
            .get_auth(token_resp.access_token.as_str())
            .unwrap();

        assert_eq!(8, auth_resp.accounts.len());

        let filtered_auth_resp = test_client
            .get_auth_with_options(
                token_resp.access_token.as_str(),
                Some(GetAuthRequestOptions {
                    account_ids: Some(vec![auth_resp.accounts[0].account_id.as_str()]),
                }),
            )
            .unwrap();

        assert_eq!(1, filtered_auth_resp.accounts.len());
    }
}
