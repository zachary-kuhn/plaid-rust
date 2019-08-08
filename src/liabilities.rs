use crate::accounts::Account;
use crate::errors::{Error, Kind};
use crate::items::Item;
use crate::plaid::Client;
use serde::*;

pub trait Liabilities {
    fn get_liabilities(&self, access_token: &str) -> Result<GetLiabilitiesResponse, Error>;
    fn get_liabilities_with_options(
        &self,
        access_token: &str,
        options: Option<GetLiabilitiesRequestOptions>,
    ) -> Result<GetLiabilitiesResponse, Error>;
}

#[derive(Deserialize)]
pub struct StudentLoanLiability {
    pub account_id: String,
    pub account_number: String,
    pub disbursement_dates: Vec<String>,
    pub expected_payoff_date: String,
    pub guarantor: String,
    pub interest_rate_percentage: f64,
    pub is_overdue: bool,
    pub last_payment_amount: f64,
    pub last_payment_date: String,
    pub last_statement_balance: f64,
    pub last_statement_issue_date: String,
    pub loan_status: StudentLoanStatus,
    pub minimum_payment_amount: f64,
    pub next_payment_due_date: String,
    pub origination_date: String,
    pub origination_principal_amount: f64,
    pub outstanding_interest_amount: f64,
    pub payment_reference_number: String,
    pub pslf_status: PSLFStatus,
    pub repayment_plan: StudentLoanRepaymentPlan,
    pub sequence_number: String,
    pub servicer_address: StudentLoanServicerAddress,
    pub ytd_interest_paid: f64,
    pub ytd_principal_paid: f64,
}

// PSLFStatus contains information about the student's eligibility in the
// Public Service Loan Forgiveness program.
#[derive(Deserialize)]
pub struct PSLFStatus {
    pub estimated_eligibility_date: String,
    pub payments_made: u64,
    pub payments_remaining: u64,
}

// StudentLoanServicerAddress is the address of the servicer.
#[derive(Deserialize)]
pub struct StudentLoanServicerAddress {
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub region: String,
    pub street: String,
}

// StudentLoanStatus contains details about the status of the student loan.
#[derive(Deserialize)]
pub struct StudentLoanStatus {
    #[serde(rename = "type")]
    pub student_loan_status_type: String,
    pub end_date: String,
}

#[derive(Serialize)]
pub struct GetLiabilitiesRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

// StudentLoanRepaymentPlan contains details about the repayment plan of the
// loan.
#[derive(Deserialize)]
pub struct StudentLoanRepaymentPlan {
    #[serde(rename = "type")]
    pub student_loand_repayment_type: String,
    pub description: String,
}

#[derive(Serialize)]
struct GetLiabilitiesRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetLiabilitiesRequestOptions>,
}

// GetLiabilitiesResponse is the response from /liabilities/get.
#[derive(Deserialize)]
pub struct GetLiabilitiesResponse {
    pub response_id: String,
    pub accounts: Vec<Account>,
    pub item: Item,
    pub liabilities: LiabilityAccounts,
}

#[derive(Deserialize)]
pub struct LiabilityAccounts {
    pub student: Vec<StudentLoanLiability>,
}

impl<'a> Liabilities for Client<'a> {
    fn get_liabilities(&self, access_token: &str) -> Result<GetLiabilitiesResponse, Error> {
        self.get_liabilities_with_options(access_token, None)
    }

    fn get_liabilities_with_options(
        &self,
        access_token: &str,
        options: Option<GetLiabilitiesRequestOptions>,
    ) -> Result<GetLiabilitiesResponse, Error> {
        if access_token == "" {
            return Err(Error::new(Kind::EmptyToken));
        }

        let req = GetLiabilitiesRequest {
            client_id: self.client_id,
            secret: self.secret,
            access_token,
            options,
        };

        serde_json::to_string(&req)
            .map_err(|err| Error::new(Kind::Json(err)))
            .and_then(|json_body| self.call("/liabilities/get", &json_body))
    }
}
