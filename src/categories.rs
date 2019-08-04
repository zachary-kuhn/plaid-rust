use crate::plaid::{Client, Error};
use serde::*;
use std::ptr::null;

trait Categories {
    fn get_categories(&self) -> Result<GetCategoriesResponse, Error>;
}

#[derive(Deserialize)]
pub struct Category {
    category_id: String,
    group: String,
    hierarchy: Vec<String>,
}

#[derive(Deserialize)]
pub struct GetCategoriesResponse {
    response_id: String,
    categories: Vec<Category>,
}

impl<'a> Categories for Client<'a> {
    fn get_categories(&self) -> Result<GetCategoriesResponse, Error> {
        self.call("/categories/get", "")
    }
}
