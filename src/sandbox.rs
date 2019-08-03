use crate::plaid::{Client, Error, Kind};
use serde::*;

pub trait Sandbox {
    fn create_sandbox_public_token(
        &self,
        institution_id: &str,
        initial_products: &[&str],
    ) -> Result<CreateSandboxPublicTokenResponse, Error>;
    fn reset_sandbox_item(&self, access_token: &str) -> Result<ResetSandboxItemResponse, Error>;
}

#[derive(Serialize)]
struct CreateSandboxPublicTokenRequest<'a> {
    institution_id: &'a str,
    initial_products: &'a [&'a str],
    public_key: &'a str,
}

#[derive(Deserialize)]
pub struct CreateSandboxPublicTokenResponse {
    response_id: String,
    public_token: String,
}

#[derive(Serialize)]
struct ResetSandboxItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct ResetSandboxItemResponse {
    response_id: String,
    reset_login: bool,
}

impl<'a> Sandbox for Client<'a> {
    fn create_sandbox_public_token(
        &self,
        institution_id: &str,
        initial_products: &[&str],
    ) -> Result<CreateSandboxPublicTokenResponse, Error> {
        if institution_id == "" || initial_products.len() == 0 {
            return Err(Error::new(Kind::ValidationError(
                "institution id and initial products must be specified",
            )));
        }

        let req = CreateSandboxPublicTokenRequest {
            institution_id,
            initial_products,
            public_key: self.public_key,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/sandbox/public_token/create", &json_body))
    }

    fn reset_sandbox_item(&self, access_token: &str) -> Result<ResetSandboxItemResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = ResetSandboxItemRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/sandbox/item/reset_login", &json_body))
    }
}
