use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::items::Item;
use crate::plaid::Client;
use serde::*;

trait Identities {
    fn get_identity(&self, access_token: &str) -> Result<GetIdentityResponse, Error>;
}

#[derive(Deserialize)]
pub struct Identity {
    pub addresses: Vec<Address>,
    pub emails: Vec<Email>,
    pub names: Vec<String>,
    pub phone_numbers: Vec<PhoneNumber>,
}

#[derive(Deserialize)]
pub struct Address {
    pub data: AddressData,
    pub primary: bool,
}

#[derive(Deserialize)]
pub struct AddressData {
    pub city: String,
    pub region: String,
    pub street: String,
    pub postal_code: String,
    pub country: Option<String>,
}

#[derive(Deserialize)]
pub struct Email {
    pub data: String,
    pub primary: bool,
    #[serde(rename = "type")]
    pub email_type: String,
}

#[derive(Deserialize)]
pub struct PhoneNumber {
    pub data: String,
    pub primary: bool,
    #[serde(rename = "type")]
    pub phone_number_type: String,
}

#[derive(Serialize)]
struct GetIdentityRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct AccountWithOwners {
    pub owners: Vec<Identity>,
    #[serde(flatten)]
    pub account: Account,
}

#[derive(Deserialize)]
pub struct GetIdentityResponse {
    pub request_id: String,
    pub accounts: Vec<AccountWithOwners>,
    pub item: Item,
}

impl<'a> Identities for Client<'a> {
    fn get_identity(&self, access_token: &str) -> Result<GetIdentityResponse, Error> {
        if access_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = GetIdentityRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/identity/get", &json_body))
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
    fn test_get_identity() {
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

        test_client
            .get_identity(token_resp.access_token.as_str())
            .unwrap();
    }
}
