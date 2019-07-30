use crate::plaid::{Client, Error, Kind};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::*;

pub trait Institutions<'a> {
    fn get_institutions_with_options(
        &self,
        count: u16,
        offset: u32,
        options: Option<GetInstitutionsRequestOptions>,
    ) -> Result<GetInstitutionsResponse, Error>;
    fn get_institutions(&self, count: u16, offset: u32) -> Result<GetInstitutionsResponse, Error>;
}

#[derive(Debug, Deserialize)]
struct Institution {
    credentials: Vec<Credential>,
    has_mfa: bool,
    institution_id: String,
    mfa: Vec<String>,
    name: String,
    products: Vec<String>,
    country_codes: Vec<String>,
    status: Option<InstitutionStatus>,
    primary_color: Option<String>,
    url: Option<String>,
    logo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Credential {
    label: String,
    name: String,
    #[serde(rename = "type")]
    credential_type: String,
}

#[derive(Debug, Deserialize)]
struct InstitutionStatus {
    item_logins: ItemLogins,
}

#[derive(Debug, Deserialize)]
struct ItemLogins {
    status: String,
    last_status_change: DateTime<Utc>,
    breakdown: InstitutionStatusBreakdown,
}

#[derive(Debug, Deserialize)]
struct InstitutionStatusBreakdown {
    success: f64,
    error_plaid: f64,
    error_institution: f64,
}

#[derive(Serialize)]
struct GetInstitutionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    count: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInstitutionsRequestOptions>,
}

#[derive(Serialize)]
pub struct GetInstitutionsRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_codes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GetInstitutionsResponse {
    request_id: String,
    institutions: Vec<Institution>,
    total: u32,
}

impl<'a> Institutions<'a> for Client<'a> {
    fn get_institutions_with_options(
        &self,
        count: u16,
        offset: u32,
        options: Option<GetInstitutionsRequestOptions>,
    ) -> Result<GetInstitutionsResponse, Error> {
        let mut result_count = count;
        if result_count == 0 {
            result_count = 50
        }

        let req = GetInstitutionsRequest {
            client_id: self.client_id,
            secret: self.secret,
            count: result_count,
            offset,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/institutions/get", &json_body))
    }

    fn get_institutions(&self, count: u16, offset: u32) -> Result<GetInstitutionsResponse, Error> {
        self.get_institutions_with_options(count, offset, None)
    }
}
