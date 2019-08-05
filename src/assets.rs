use crate::accounts::Account;
use crate::plaid::{Client, Error, Kind};
use serde::*;

trait Assets {
    fn get_asset_report(&self, asset_report_token: &str) -> Result<GetAssetReportResponse, Error>;
}

#[derive(Deserialize)]
pub struct AssetReport {
    asset_report_id: String,
    client_report_id: String,
    date_generated: String,
    days_requested: i64,
    items: Vec<AssetReportItem>,
    user: AssetReportUser,
}

#[derive(Deserialize)]
pub struct AssetReportItem {
    accounts: Vec<Account>,
    date_last_updated: String,
    institution_id: String,
    institution_name: String,
    item_id: String,
}

#[derive(Deserialize)]
pub struct AssetReportUser {
    client_user_id: String,
    email: String,
    first_name: String,
    last_name: String,
    middle_name: String,
    phone_number: String,
    ssn: String,
}

#[derive(Serialize)]
struct GetAssetReportRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetAssetReportResponse {
    response_id: String,
    report: AssetReport,
    warnings: Vec<String>,
}

impl<'a> Assets for Client<'a> {
    fn get_asset_report(&self, asset_report_token: &str) -> Result<GetAssetReportResponse, Error> {
        if asset_report_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = GetAssetReportRequest {
            client_id: self.client_id,
            secret: self.secret,
            asset_report_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/asset_report/get", &json_body))
    }
}
