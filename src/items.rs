use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Items {
    fn get_item(&self, access_token: &str) -> Result<GetItemResponse, Error>;
}

#[derive(Deserialize)]
pub struct Item {
    available_products: Vec<String>,
    billed_products: Vec<String>,
    error: String,
    institution_id: String,
    item_id: String,
    webhook: String,
}

#[derive(Serialize)]
struct GetItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetItemResponse {
    response_id: String,
    item: Item,
}

impl<'a> Items for Client<'a> {
    fn get_item(&self, access_token: &str) -> Result<GetItemResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetItemRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/get", &json_body))
    }
}
