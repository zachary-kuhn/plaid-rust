use crate::plaid::Client;
use crate::plaid::Error;
use serde::Deserialize;

pub trait Auth {
    fn getAuthWithOptions(
        &self,
        access_token: &'static str,
        options: GetAuthRequestOptions,
    ) -> Result<GetAuthRequest, Error>;
    fn getAuth(&self, access_token: &'static str) -> Result<GetAuthRequest, Error>;
}

#[derive(Deserialize)]
struct GetAuthRequestOptions {
    account_ids: Vec<str>,
}

struct GetAuthRequest {
    client_id: &'static str,
    secret: &'static str,
    access_token: &'static str,
    options: GetAuthRequestOptions,
}

struct GetAuthResponse {
    request_id: str,
    accounts: Vec<str>,
    numbers: Vec<str>,
}

impl Auth for Client {
    fn getAuthWithOptions(
        &self,
        access_token: &'static str,
        options: GetAuthRequestOptions,
    ) -> Result<GetAuthResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        req = GetAuthRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token: self.access_token,
            options: GetAuthRequestOptions {
                account_ids: options.account_ids,
            },
        };

        serde_json::to_string(req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .map(|json_body| self.call("/auth/get", &json_body))
    }

    fn getAuth(&self, access_token: &'static str) -> Result<GetAuthResponse, Error> {
        self.getAuthWithOptions(
            access_token,
            GetAuthRequestOptions {
                account_ids: vec![],
            },
        )
    }
}
