pub enum Environment {
    SANDBOX,
    DEVELOPMENT,
    PRODUCTION,
}

impl Environment {
    pub fn host(&self) -> &str {
        match self {
            Environment::SANDBOX => "sandbox.plaid.com",
            Environment::DEVELOPMENT => "development.plaid.com",
            Environment::PRODUCTION => "production.plaid.com",
        }
    }
}
