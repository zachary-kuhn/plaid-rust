use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::plaid::Client;
use serde::*;

trait Assets {
    fn get_asset_report(&self, asset_report_token: &str) -> Result<GetAssetReportResponse, Error>;
    fn create_audit_copy(
        &self,
        asset_report_token: &str,
        auditor_id: &str,
    ) -> Result<CreateAuditCopyTokenResponse, Error>;
    fn remove_asset_report(
        &self,
        asset_report_token: &str,
    ) -> Result<RemoveAssetReportResponse, Error>;
}

#[derive(Deserialize)]
pub struct AssetReport {
    pub asset_report_id: String,
    pub client_report_id: String,
    pub date_generated: String,
    pub days_requested: i64,
    pub items: Vec<AssetReportItem>,
    pub user: AssetReportUser,
}

#[derive(Deserialize)]
pub struct AssetReportItem {
    pub accounts: Vec<Account>,
    pub date_last_updated: String,
    pub institution_id: String,
    pub institution_name: String,
    pub item_id: String,
}

#[derive(Deserialize)]
pub struct AssetReportUser {
    pub client_user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub phone_number: String,
    pub ssn: String,
}

#[derive(Serialize)]
struct GetAssetReportRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
}

#[derive(Deserialize)]
pub struct GetAssetReportResponse {
    pub response_id: String,
    pub report: AssetReport,
    pub warnings: Vec<String>,
}

#[derive(Serialize)]
struct CreateAuditCopyRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
    auditor_id: &'a str,
}

#[derive(Deserialize)]
pub struct CreateAuditCopyTokenResponse {
    pub response_id: String,
    pub audit_copy_token: String,
}

#[derive(Serialize)]
struct RemoveAssetReportRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
}

#[derive(Deserialize)]
pub struct RemoveAssetReportResponse {
    pub response_id: String,
    pub removed: bool,
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

    fn create_audit_copy(
        &self,
        asset_report_token: &str,
        auditor_id: &str,
    ) -> Result<CreateAuditCopyTokenResponse, Error> {
        if asset_report_token == "" || auditor_id == "" {
            Err(Error::new(Kind::ValidationError(
                "asset report token and auditor id must be specified",
            )))?
        }

        let req = CreateAuditCopyRequest {
            client_id: self.client_id,
            secret: self.secret,
            asset_report_token,
            auditor_id,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/asset_report/audit_copy/create", &json_body))
    }

    fn remove_asset_report(
        &self,
        asset_report_token: &str,
    ) -> Result<RemoveAssetReportResponse, Error> {
        if asset_report_token == "" {
            Err(Error::new(Kind::EmptyToken))?
        }

        let req = RemoveAssetReportRequest {
            client_id: self.client_id,
            secret: self.secret,
            asset_report_token,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/asset_report/remove", &json_body))
    }
}
