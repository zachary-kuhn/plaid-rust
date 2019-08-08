use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::items::Item;
use crate::plaid::Client;
use serde::*;

trait Transactions {
    fn get_transactions(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<GetTransactionsResponse, Error>;
    fn get_transactions_with_options(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
        options: Option<GetTransactionsRequestOptions>,
    ) -> Result<GetTransactionsResponse, Error>;
}

#[derive(Deserialize)]
pub struct Transaction {
    pub account_id: String,
    pub amount: f64,
    pub iso_currency_code: String,
    pub unofficial_currency_code: String,
    pub category: Vec<String>,
    pub category_id: String,
    pub date: String,
    pub location: Location,
    pub name: String,
    pub payment_meta: PaymentMeta,
    pub pending: bool,
    pub pending_transaction_id: String,
    pub account_owner: String,
    pub transaction_id: String,
    pub transaction_type: String,
}

#[derive(Deserialize)]
pub struct Location {
    pub address: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
    pub region: String,
    pub store_number: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Deserialize)]
pub struct PaymentMeta {
    pub by_order_of: String,
    pub payee: String,
    pub payer: String,
    pub payment_method: String,
    pub payment_processor: String,
    pub ppd_id: String,
    pub reason: String,
    pub reference_number: String,
}

#[derive(Serialize)]
struct GetTransactionsRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i64>,
}

#[derive(Serialize)]
struct GetTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    start_date: &'a str,
    end_date: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetTransactionsRequestOptions<'a>>,
}

#[derive(Deserialize)]
pub struct GetTransactionsResponse {
    pub response_id: String,
    pub accounts: Vec<Account>,
    pub item: Item,
    pub transactions: Vec<Transaction>,
    pub total_transactions: i64,
}

impl<'a> Transactions for Client<'a> {
    fn get_transactions(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<GetTransactionsResponse, Error> {
        self.get_transactions_with_options(access_token, start_date, end_date, None)
    }

    fn get_transactions_with_options(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
        options: Option<GetTransactionsRequestOptions>,
    ) -> Result<GetTransactionsResponse, Error> {
        if start_date == "" || end_date == "" {
            Err(Error::new(Kind::ValidationError(
                "start date and end date must be specified",
            )))?
        }

        let req = GetTransactionsRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            start_date,
            end_date,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/transactions/get", &json_body))
    }
}
