use serde::{Deserialize, Serialize};
use validator::Validate;
use mysql_async::prelude::FromRow;
use mysql_async::Row;

#[derive(Serialize)]
pub struct OutgoingIdentifier{
    pub concatenated_string: String,
    pub color: String,
    pub product_name: String,
    pub warehouse: String,
    pub location: String,
    pub pcs: i32,
}

impl FromRow for OutgoingIdentifier{
    fn from_row(row: Row) -> Self{

        let (concatenated_string, color, product_name, warehouse, location, pcs):(String, String, String, String, String, i32) = mysql_async::from_row(row);

        OutgoingIdentifier{concatenated_string, color, product_name,warehouse,location,pcs}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (concatenated_string, color, product_name, warehouse, location, pcs):(String, String, String, String, String, i32) = mysql_async::from_row(row);

        Ok(OutgoingIdentifier{concatenated_string, color, product_name,warehouse,location,pcs})
    }
}


//adding unique outgoing identifier request
#[derive(Validate, Deserialize, Serialize)]
pub struct RemoveUniqueIdentifierRequest{
    #[validate(length(min =1, message = "Color is required"))]
    pub color: String,
    #[validate(length(min =1, message = "Product name is required"))]
    pub product_name: String,
    #[validate(length(min =1, message = "Warehouse is required"))]
    pub warehouse: String,
    #[validate(length(min =1, message = "Location is required"))]
    pub location: String,
    #[validate(range(min = 1, max = 10000, message = "PCS must be between 1 and 100"))]
    pub pcs: i32,
}