use crate::plaid::{Client, Error, Kind};
use serde::{Deserialize, Serialize};

pub trait Auth<'a> {
    fn get_auth_with_options(
        &self,
        access_token: &'a str,
        options: GetAuthRequestOptions,
    ) -> Result<GetAuthResponse, Error>;
    fn get_auth(&self, access_token: &'a str) -> Result<GetAuthResponse, Error>;
}

#[derive(Serialize)]
struct GetAuthRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    options: GetAuthRequestOptions,
}

#[derive(Serialize)]
pub struct GetAuthRequestOptions {
    account_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct GetAuthResponse {
    pub request_id: String,
    pub accounts: Vec<String>,
    pub numbers: Vec<String>,
}

impl<'a> Auth<'a> for Client<'a> {
    fn get_auth_with_options(
        &self,
        access_token: &str,
        options: GetAuthRequestOptions,
    ) -> Result<GetAuthResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetAuthRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options: GetAuthRequestOptions {
                account_ids: options.account_ids,
            },
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/auth/get", &json_body))
    }

    fn get_auth(&self, access_token: &str) -> Result<GetAuthResponse, Error> {
        self.get_auth_with_options(
            access_token,
            GetAuthRequestOptions {
                account_ids: vec![],
            },
        )
    }
}
