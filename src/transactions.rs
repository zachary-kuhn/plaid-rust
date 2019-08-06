use crate::accounts::Account;
use crate::items::Item;
use crate::plaid::{Client, Error, Kind};
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
    account_id: String,
    amount: f64,
    iso_currency_code: String,
    unofficial_currency_code: String,
    category: Vec<String>,
    category_id: String,
    date: String,
    location: Location,
    name: String,
    payment_meta: PaymentMeta,
    pending: bool,
    pending_transaction_id: String,
    account_owner: String,
    transaction_id: String,
    transaction_type: String,
}

#[derive(Deserialize)]
pub struct Location {
    address: String,
    city: String,
    lat: f64,
    lon: f64,
    region: String,
    store_number: String,
    postal_code: String,
    country: String,
}

#[derive(Deserialize)]
pub struct PaymentMeta {
    by_order_of: String,
    payee: String,
    payer: String,
    payment_method: String,
    payment_processor: String,
    ppd_id: String,
    reason: String,
    reference_number: String,
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
    response_id: String,
    accounts: Vec<Account>,
    item: Item,
    transactions: Vec<Transaction>,
    total_transactions: i64,
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
