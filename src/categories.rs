use crate::plaid::{Client, Error};
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
    pub response_id: String,
    pub categories: Vec<Category>,
}

impl<'a> Categories for Client<'a> {
    fn get_categories(&self) -> Result<GetCategoriesResponse, Error> {
        self.call("/categories/get", "")
    }
}
