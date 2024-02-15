use serde::{Deserialize, Serialize};
use validator::Validate;
use mysql_async::prelude::FromRow;
use mysql_async::Row;


#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct SalesOrder{
    #[validate(length(min =1, message = "Order Number is required"))]
    pub order_number: String,
    #[validate(length(min =1, message = "Product Code is required"))]
    pub product_code: String,
    #[validate(length(min =1, message = "Color is required"))]
    pub color: String,
    #[validate(length(min =1, message = "Product Name is required"))]
    pub product_name: String,
    #[validate(range(min = 1, max = 10000, message = "PCS must be between 1 and 100"))]
    pub pcs: i32,
    #[validate(length(min =1, message = "Company Name is required"))]
    pub company: String,
}

impl FromRow for SalesOrder {
    fn from_row(row: Row) -> Self{

        let (order_number, product_code, color, product_name,pcs, company):(String, String,String, String, i32, String) = mysql_async::from_row(row);

        SalesOrder{order_number, product_code, color, product_name,pcs, company}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (order_number, product_code,color, product_name, pcs, company):(String, String,String, String, i32, String) = mysql_async::from_row(row);

        Ok(SalesOrder{order_number, product_code,color, product_name,pcs, company})
    }
    
    
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SalesOrderProduct {
    pub product_code: Option<String>,
    pub color: Option<String>,
    pub product_name: Option<String>,
    pub warehouse: Option<String>,
    pub location: Option<String>,
    pub warehouse_pcs: Option<i32>,
    pub order_pcs: Option<i32>,
    pub deducted_pcs: Option<i32>,
    pub difference: Option<i32>,
    
}

impl FromRow for SalesOrderProduct {
    fn from_row(row: Row) -> Self{

        let (product_code, color, product_name, warehouse, location, warehouse_pcs, order_pcs, deducted_pcs, difference):(Option<String>,Option<String>, Option<String>, Option<String>, Option<String>,  Option<i32> , Option<i32>, Option<i32>, Option<i32>) = mysql_async::from_row(row);

        SalesOrderProduct{product_code, color, product_name, warehouse, location, warehouse_pcs, order_pcs, deducted_pcs, difference}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (product_code, color, product_name, warehouse, location, warehouse_pcs, order_pcs, deducted_pcs, difference):(Option<String>,Option<String>,Option<String>,Option<String>,Option<String>, Option<i32>, Option<i32>,  Option<i32>,Option<i32>) = mysql_async::from_row(row);

        Ok(SalesOrderProduct{product_code, color, product_name, warehouse, location, warehouse_pcs, order_pcs, deducted_pcs, difference})
    }
    
    
}

//adding get request for single product location
#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct GetSalesOrder{
    #[validate(length(min =1, message = "Order Number is required"))]
    pub order_number: String,
}