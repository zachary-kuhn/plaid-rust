use crate::errors::{Error, Kind};
use crate::plaid::Client;
use serde::*;

trait Incomes {
    fn get_income(&self, access_token: &str) -> Result<GetIncomeResponse, Error>;
}

#[derive(Deserialize)]
pub struct Income {
    pub income_streams: Vec<IncomeStream>,
    pub last_year_income: i64,
    pub last_year_income_before_tax: i64,
    pub projected_yearly_income: i64,
    pub projected_yearly_income_before_tax: i64,
    pub max_number_of_overlapping_income_streams: i64,
    pub number_of_income_streams: i64,
}

#[derive(Deserialize)]
pub struct IncomeStream {
    pub confidence: f64,
    pub days: i64,
    pub monthly_income: i64,
    pub name: String,
}

#[derive(Serialize)]
struct GetIncomeRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetIncomeResponse {
    pub response_id: String,
    pub income: Income,
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
