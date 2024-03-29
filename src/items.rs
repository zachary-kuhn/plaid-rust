use crate::errors::{Error, Kind};
use crate::plaid::Client;
use serde::*;

pub trait Items {
    fn get_item(&self, access_token: &str) -> Result<GetItemResponse, Error>;
    fn remove_item(&self, access_token: &str) -> Result<RemoveItemResponse, Error>;
    fn update_item_webhook(
        &self,
        access_token: &str,
        webhook: &str,
    ) -> Result<UpdateItemWebhookResponse, Error>;
    fn invalidate_access_token(
        &self,
        access_token: &str,
    ) -> Result<InvalidateAccessTokenResponse, Error>;
    fn update_access_token_version(
        &self,
        access_token: &str,
    ) -> Result<UpdateAccessTokenVersionResponse, Error>;
    fn create_public_token(&self, access_token: &str) -> Result<CreatePublicTokenResponse, Error>;
    fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> Result<ExchangePublicTokenResponse, Error>;
}

#[derive(Deserialize)]
pub struct Item {
    pub available_products: Vec<String>,
    pub billed_products: Vec<String>,
    pub error: Option<ItemError>,
    pub institution_id: String,
    pub item_id: String,
    pub webhook: Option<String>,
}

#[derive(Deserialize)]
pub struct ItemError {
    pub error_type: String,
    pub error_code: String,
    pub error_message: String,
    pub display_message: Option<String>,
}

#[derive(Serialize)]
struct GetItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetItemResponse {
    pub request_id: String,
    pub item: Item,
}

#[derive(Serialize)]
struct RemoveItemRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct RemoveItemResponse {
    pub request_id: String,
    pub removed: bool,
}

#[derive(Serialize)]
struct UpdateItemWebhookRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    webhook: &'a str,
}

#[derive(Deserialize)]
pub struct UpdateItemWebhookResponse {
    pub request_id: String,
    pub item: Item,
}

#[derive(Serialize)]
struct InvalidateAccessTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct InvalidateAccessTokenResponse {
    pub request_id: String,
    pub new_access_token: String,
}

#[derive(Serialize)]
struct UpdateAccessTokenVersionRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    #[serde(rename = "access_token_v1")]
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct UpdateAccessTokenVersionResponse {
    pub request_id: String,
    pub access_token: String,
    pub item_id: String,
}

#[derive(Serialize)]
struct CreatePublicTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct CreatePublicTokenResponse {
    pub request_id: String,
    pub public_token: String,
}

#[derive(Serialize)]
struct ExchangePublicTokenRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    public_token: &'a str,
}

#[derive(Deserialize)]
pub struct ExchangePublicTokenResponse {
    pub request_id: String,
    pub access_token: String,
    pub item_id: String,
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

    fn remove_item(&self, access_token: &str) -> Result<RemoveItemResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = RemoveItemRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/remove", &json_body))
    }

    fn update_item_webhook(
        &self,
        access_token: &str,
        webhook: &str,
    ) -> Result<UpdateItemWebhookResponse, Error> {
        if access_token == "" || webhook == "" {
            return Err(Error::new(Kind::ValidationError(
                "access token and webhook must be specified",
            )));
        }

        let req = UpdateItemWebhookRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            webhook,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/webhook/update", &json_body))
    }

    fn invalidate_access_token(
        &self,
        access_token: &str,
    ) -> Result<InvalidateAccessTokenResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = InvalidateAccessTokenRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/access_token/invalidate", &json_body))
    }

    fn update_access_token_version(
        &self,
        access_token: &str,
    ) -> Result<UpdateAccessTokenVersionResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = UpdateAccessTokenVersionRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/access_token/update_version", &json_body))
    }

    fn create_public_token(&self, access_token: &str) -> Result<CreatePublicTokenResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = CreatePublicTokenRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/public_token/create", &json_body))
    }

    fn exchange_public_token(
        &self,
        public_token: &str,
    ) -> Result<ExchangePublicTokenResponse, Error> {
        if public_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = ExchangePublicTokenRequest {
            client_id: self.client_id,
            secret: self.secret,
            public_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/item/public_token/exchange", &json_body))
    }
}
