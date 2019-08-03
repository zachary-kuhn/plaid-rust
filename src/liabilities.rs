use crate::accounts::Account;
use crate::items::Item;
use crate::plaid::{Client, Error, Kind};
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
    account_id: String,
    account_number: String,
    disbursement_dates: Vec<String>,
    expected_payoff_date: String,
    guarantor: String,
    interest_rate_percentage: f64,
    is_overdue: bool,
    last_payment_amount: f64,
    last_payment_date: String,
    last_statement_balance: f64,
    last_statement_issue_date: String,
    loan_status: StudentLoanStatus,
    minimum_payment_amount: f64,
    next_payment_due_date: String,
    origination_date: String,
    origination_principal_amount: f64,
    outstanding_interest_amount: f64,
    payment_reference_number: String,
    pslf_status: PSLFStatus,
    repayment_plan: StudentLoanRepaymentPlan,
    sequence_number: String,
    servicer_address: StudentLoanServicerAddress,
    ytd_interest_paid: f64,
    ytd_principal_paid: f64,
}

// PSLFStatus contains information about the student's eligibility in the
// Public Service Loan Forgiveness program.
#[derive(Deserialize)]
pub struct PSLFStatus {
    estimated_eligibility_date: String,
    payments_made: u64,
    payments_remaining: u64,
}

// StudentLoanServicerAddress is the address of the servicer.
#[derive(Deserialize)]
pub struct StudentLoanServicerAddress {
    city: String,
    country: String,
    postal_code: String,
    region: String,
    street: String,
}

// StudentLoanStatus contains details about the status of the student loan.
#[derive(Deserialize)]
pub struct StudentLoanStatus {
    #[serde(rename = "struct")]
    student_loan_status_type: String,
    end_date: String,
}

#[derive(Serialize)]
pub struct GetLiabilitiesRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_ids: Option<Vec<String>>,
}

// StudentLoanRepaymentPlan contains details about the repayment plan of the
// loan.
#[derive(Deserialize)]
pub struct StudentLoanRepaymentPlan {
    #[serde(rename = "struct")]
    student_loand_repayment_type: String,
    description: String,
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
    response_id: String,
    accounts: Vec<Account>,
    item: Item,
    liabilities: LiabilityAccounts,
}

#[derive(Deserialize)]
pub struct LiabilityAccounts {
    student: Vec<StudentLoanLiability>,
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
