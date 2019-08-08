use crate::errors::{Error, Kind};
use crate::plaid::Client;
use chrono::{DateTime, Utc};
use serde::*;

pub trait Institutions {
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
pub struct Institution {
    pub credentials: Vec<Credential>,
    pub has_mfa: bool,
    pub institution_id: String,
    pub mfa: Vec<String>,
    pub name: String,
    pub products: Vec<String>,
    pub country_codes: Vec<String>,
    pub status: Option<InstitutionStatus>,
    pub primary_color: Option<String>,
    pub url: Option<String>,
    pub logo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Credential {
    pub label: String,
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: String,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionStatus {
    pub item_logins: ItemLogins,
}

#[derive(Debug, Deserialize)]
pub struct ItemLogins {
    pub status: String,
    pub last_status_change: DateTime<Utc>,
    pub breakdown: InstitutionStatusBreakdown,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionStatusBreakdown {
    pub success: f64,
    pub error_plaid: f64,
    pub error_institution: f64,
}

#[derive(Serialize)]
struct GetInstitutionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    count: u16,
    offset: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInstitutionsRequestOptions<'a>>,
}

#[derive(Serialize)]
pub struct GetInstitutionsRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    products: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_codes: Option<Vec<&'a str>>,
}

#[derive(Debug, Deserialize)]
pub struct GetInstitutionsResponse {
    pub request_id: String,
    pub institutions: Vec<Institution>,
    pub total: u32,
}

#[derive(Serialize)]
struct GetInstitutionByIdRequest<'a> {
    institution_id: &'a str,
    public_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInstitutionByIdRequestOptions>,
}

#[derive(Serialize)]
pub struct GetInstitutionByIdRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_status: Option<bool>,
}

#[derive(Deserialize)]
pub struct GetInstitutionByIdResponse {
    pub request_id: String,
    pub institution: Institution,
}

#[derive(Serialize)]
struct SearchInstitutionsRequest<'a> {
    query: &'a str,
    products: Vec<&'a str>,
    public_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<SearchInstitutionsRequestOptions<'a>>,
}

#[derive(Serialize)]
pub struct SearchInstitutionsRequestOptions<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_optional_metadata: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_codes: Option<Vec<&'a str>>,
}

#[derive(Deserialize)]
pub struct SearchInstitutionsResponse {
    pub request_id: String,
    pub institutions: Vec<Institution>,
}

impl<'a> Institutions for Client<'a> {
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
