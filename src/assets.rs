use crate::accounts::Account;
use crate::plaid::{Client, Error, Kind};
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

#[derive(Serialize)]
struct CreateAuditCopyRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
    auditor_id: &'a str,
}

#[derive(Deserialize)]
pub struct CreateAuditCopyTokenResponse {
    response_id: String,
    audit_copy_token: String,
}

#[derive(Serialize)]
struct RemoveAssetReportRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    asset_report_token: &'a str,
}

#[derive(Deserialize)]
pub struct RemoveAssetReportResponse {
    response_id: String,
    removed: bool,
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
