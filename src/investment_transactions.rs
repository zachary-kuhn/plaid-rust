use crate::accounts::Account;
use crate::holdings::Security;
use crate::items::Item;
use crate::plaid::{Client, Error, Kind};
use serde::*;

trait InvestmentTransactions {
    fn get_investment_transactions(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<GetInvestmentTransactionsResponse, Error>;
    fn get_investment_transactions_with_options(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
        options: Option<GetInvestmentTransactionsRequestOptions>,
    ) -> Result<GetInvestmentTransactionsResponse, Error>;
}

#[derive(Deserialize)]
pub struct InvestmentTransaction {
    investment_transaction_id: String,
    account_id: String,
    security_id: String,
    cancel_transaction_id: String,
    date: String,
    name: String,
    quantity: f64,
    amount: f64,
    price: f64,
    fees: f64,
    #[serde(rename = "type")]
    investment_transaction_type: String,
    iso_currency_code: String,
    unofficial_currency_code: String,
}

#[derive(Serialize)]
struct GetInvestmentTransactionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    start_date: &'a str,
    end_date: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInvestmentTransactionsRequestOptions<'a>>,
}

#[derive(Serialize)]
struct GetInvestmentTransactionsRequestOptions<'a> {
    account_ids: Option<Vec<&'a str>>,
    count: Option<i64>,
    offsent: Option<i64>,
}

#[derive(Deserialize)]
pub struct GetInvestmentTransactionsResponse {
    response_id: String,
    item: Item,
    accounts: Vec<Account>,
    investment_transactions: Vec<InvestmentTransaction>,
    securities: Vec<Security>,
    total_investment_transactions: i64,
}

impl<'a> InvestmentTransactions for Client<'a> {
    fn get_investment_transactions(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<GetInvestmentTransactionsResponse, Error> {
        self.get_investment_transactions_with_options(access_token, start_date, end_date, None)
    }

    fn get_investment_transactions_with_options(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
        options: Option<GetInvestmentTransactionsRequestOptions>,
    ) -> Result<GetInvestmentTransactionsResponse, Error> {
        if start_date == "" || end_date == "" {
            Err(Error::new(Kind::ValidationError(
                "start date and end date must be specified",
            )))?
        }

        let req = GetInvestmentTransactionsRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            start_date,
            end_date,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/investments/transactions/get", &json_body))
    }
}
