use crate::plaid::{Client, Error, Kind};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::*;

pub trait Institutions<'a> {
    fn get_institutions(&self, count: u16, offset: u32) -> Result<GetInstitutionsResponse, Error>;
    fn get_institutions_with_options(
        &self,
        count: u16,
        offset: u32,
        options: Option<GetInstitutionsRequestOptions>,
    ) -> Result<GetInstitutionsResponse, Error>;
    fn search_institutions(
        &self,
        query: &str,
        products: Vec<&str>,
    ) -> Result<SearchInstitutionsResponse, Error>;
    fn search_institutions_with_options(
        &self,
        query: &str,
        products: Vec<&str>,
        options: Option<SearchInstitutionsRequestOptions>,
    ) -> Result<SearchInstitutionsResponse, Error>;
    fn get_institution_by_id(&self, id: &str) -> Result<GetInstitutionByIdResponse, Error>;
    fn get_institution_by_id_with_options(
        &self,
        id: &str,
        options: Option<GetInstitutionByIdRequestOptions>,
    ) -> Result<GetInstitutionByIdResponse, Error>;
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

#[derive(Serialize)]
struct GetInstitutionByIdRequest<'a> {
    institution_id: &'a str,
    public_key: &'a str,
    options: Option<GetInstitutionByIdRequestOptions>,
}

#[derive(Serialize)]
pub struct GetInstitutionByIdRequestOptions {
    include_optional_metadata: bool,
    include_status: bool,
}

#[derive(Deserialize)]
pub struct GetInstitutionByIdResponse {
    request_id: String,
    institution: Institution,
}

#[derive(Serialize)]
struct SearchInstitutionsRequest<'a> {
    query: &'a str,
    products: Vec<&'a str>,
    public_key: &'a str,
    options: Option<SearchInstitutionsRequestOptions<'a>>,
}

#[derive(Serialize)]
pub struct SearchInstitutionsRequestOptions<'a> {
    include_optional_metadata: bool,
    country_codes: Vec<&'a str>,
}

#[derive(Deserialize)]
pub struct SearchInstitutionsResponse {
    request_id: String,
    institutions: Vec<Institution>,
}

impl<'a> Institutions<'a> for Client<'a> {
    fn get_institutions(&self, count: u16, offset: u32) -> Result<GetInstitutionsResponse, Error> {
        self.get_institutions_with_options(count, offset, None)
    }

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

    fn search_institutions(
        &self,
        query: &str,
        products: Vec<&str>,
    ) -> Result<SearchInstitutionsResponse, Error> {
        self.search_institutions_with_options(query, products, None)
    }

    fn search_institutions_with_options(
        &self,
        query: &str,
        products: Vec<&str>,
        options: Option<SearchInstitutionsRequestOptions>,
    ) -> Result<SearchInstitutionsResponse, Error> {
        if query == "" {
            return Err(Error::new(Kind::EmptyQuery));
        }

        let req = SearchInstitutionsRequest {
            query,
            products,
            public_key: self.public_key,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/institutions/search", &json_body))
    }

    fn get_institution_by_id(&self, id: &str) -> Result<GetInstitutionByIdResponse, Error> {
        self.get_institution_by_id_with_options(id, None)
    }

    fn get_institution_by_id_with_options(
        &self,
        id: &str,
        options: Option<GetInstitutionByIdRequestOptions>,
    ) -> Result<GetInstitutionByIdResponse, Error> {
        if id == "" {
            return Err(Error::new(Kind::EmptyId));
        }

        let req = GetInstitutionByIdRequest {
            institution_id: id,
            public_key: self.public_key,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/institutions/get_by_id", &json_body))
    }
}
