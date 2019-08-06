use crate::accounts::Account;
use crate::items::Item;
use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Identities {
    fn get_identity(&self, access_token: &str) -> Result<GetIdentityResponse, Error>;
}

#[derive(Deserialize)]
pub struct Identity {
    addresses: Vec<Address>,
    emails: Vec<Email>,
    names: Vec<String>,
    phone_numbers: Vec<PhoneNumber>,
}

#[derive(Deserialize)]
pub struct Address {
    data: AddressData,
    primary: bool,
}

#[derive(Deserialize)]
pub struct AddressData {
    city: String,
    region: String,
    street: String,
    postal_code: String,
    country: String,
}

#[derive(Deserialize)]
pub struct Email {
    data: String,
    primary: bool,
    #[serde(rename = "type")]
    email_type: String,
}

#[derive(Deserialize)]
pub struct PhoneNumber {
    data: String,
    primary: bool,
    #[serde(rename = "type")]
    phone_number_type: String,
}

#[derive(Serialize)]
struct GetIdentityRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct AccountWithOwners {
    owners: Vec<Identity>,
    #[serde(flatten)]
    account: Account,
}

#[derive(Deserialize)]
pub struct GetIdentityResponse {
    response_id: String,
    accounts: Vec<AccountWithOwners>,
    item: Item,
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
