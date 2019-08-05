use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Processors {
    fn create_apex_token(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<CreateApexTokenResponse, Error>;
}

#[derive(Serialize)]
struct CreateApexTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    account_id: &'a str,
}

#[derive(Deserialize)]
pub struct CreateApexTokenResponse {
    response_id: String,
    processor_token: String,
}

impl<'a> Processors for Client<'a> {
    fn create_apex_token(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> Result<CreateApexTokenResponse, Error> {
        if access_token == "" || account_id == "" {
            Err(Error::new(Kind::ValidationError(
                "access token and account ID must be specified",
            )))?
        }

        let req = CreateApexTokenRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            account_id,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/processor/apex/processor_token/create", &json_body))
    }
}