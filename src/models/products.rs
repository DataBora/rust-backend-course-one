use serde::{Deserialize, Serialize};
use validator::Validate;
use mysql_async::prelude::FromRow;
use mysql_async::Row;


#[derive(Serialize, Debug)]
pub struct Products{
    pub product_code: String,
    pub color: String,
    pub product_name: String,
}

impl FromRow for Products {
    fn from_row(row: Row) -> Self{

        let (product_code, color, product_name):(String, String, String) = mysql_async::from_row(row);

        Products{product_code, color, product_name}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (product_code, color, product_name):(String, String, String) = mysql_async::from_row(row);

        Ok(Products{product_code, color, product_name})
    }

}

#[derive(Validate, Deserialize, Serialize)]
pub struct AddProductCodeToUniqueIdentifiers{
    #[validate(length(min =1, message = "Color is required"))]
    pub color: String,
    #[validate(length(min =1, message = "Product name is required!"))]
    pub product_name: String,
}

