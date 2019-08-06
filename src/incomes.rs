use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Incomes {
    fn get_income(&self, access_token: &str) -> Result<GetIncomeResponse, Error>;
}

#[derive(Deserialize)]
pub struct Income {
    income_streams: Vec<IncomeStream>,
    last_year_income: i64,
    last_year_income_before_tax: i64,
    projected_yearly_income: i64,
    projected_yearly_income_before_tax: i64,
    max_number_of_overlapping_income_streams: i64,
    number_of_income_streams: i64,
}

#[derive(Deserialize)]
pub struct IncomeStream {
    confidence: f64,
    days: i64,
    monthly_income: i64,
    name: String,
}

#[derive(Serialize)]
struct GetIncomeRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetIncomeResponse {
    response_id: String,
    income: Income,
}

impl<'a> Incomes for Client<'a> {
    fn get_income(&self, access_token: &str) -> Result<GetIncomeResponse, Error> {
        if access_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = GetIncomeRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/income/get", &json_body))
    }
}
