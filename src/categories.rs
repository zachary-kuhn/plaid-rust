use crate::errors::Error;
use crate::plaid::Client;
use serde::*;

trait Categories {
    fn get_categories(&self) -> Result<GetCategoriesResponse, Error>;
}

#[derive(Deserialize)]
pub struct Category {
    pub category_id: String,
    pub group: String,
    pub hierarchy: Vec<String>,
}

#[derive(Deserialize)]
pub struct GetCategoriesResponse {
    pub request_id: String,
    pub categories: Vec<Category>,
}

impl<'a> Categories for Client<'a> {
    fn get_categories(&self) -> Result<GetCategoriesResponse, Error> {
        self.call("/categories/get", "null")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environments::Environment;
    use std::env;

    #[test]
    fn test_get_categories() {
        let client_id = env::var("PLAID_CLIENT_ID").unwrap();
        let secret = env::var("PLAID_SECRET").unwrap();
        let public_key = env::var("PLAID_PUBLIC_KEY").unwrap();
        let test_client = Client {
            client_id: client_id.as_str(),
            secret: secret.as_str(),
            public_key: public_key.as_str(),
            environment: Environment::SANDBOX,
            http_client: reqwest::Client::new(),
        };

        let categories_resp = test_client.get_categories().unwrap();

        assert_eq!("10000000", categories_resp.categories[0].category_id);
        assert_eq!("special", categories_resp.categories[0].group);
    }
}
