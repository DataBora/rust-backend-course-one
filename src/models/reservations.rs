use serde::{Deserialize, Serialize};
use validator::Validate;
use mysql_async::prelude::FromRow;
use mysql_async::Row;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Reservations{
    pub order_number: String,
    pub product_code: String,
    pub reservation_warehouse: String,
    pub reservation_location: String,
    pub pcs: i32
}

impl FromRow for Reservations {
    fn from_row(row: Row) -> Self{

        let (order_number, product_code, reservation_warehouse, reservation_location, pcs):(String, String, String, String, i32) = mysql_async::from_row(row);

        Reservations{order_number, product_code, reservation_warehouse, reservation_location, pcs}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (order_number, product_code, reservation_warehouse, reservation_location, pcs):(String, String, String, String, i32) = mysql_async::from_row(row);

        Ok(Reservations{order_number, product_code, reservation_warehouse, reservation_location, pcs})
    }

}

#[derive(Validate, Deserialize, Serialize)]
pub struct AddReservationForOrderNumber{
    #[validate(length(min =1, message = "Order Number is required"))]
    pub order_number: String,
    #[validate(length(min =1, message = "Product Code is required"))]
    pub product_code: String,
    #[validate(length(min =1, message = "Current Warehouse is required"))]
    pub current_warehouse: String,
    #[validate(length(min =1, message = "Current Location is required"))]
    pub current_location: String,
    #[validate(length(min =1, message = "Reservation Warehouse is required"))]
    pub reservation_warehouse: String,
    #[validate(length(min =1, message = "reservation Location is required"))]
    pub reservation_location: String,
    #[validate(range(min =0, max=10000, message = "Pcs are required and must be between 0 and 10000"))]
    pub pcs: i32
}

#[derive(Validate, Deserialize, Serialize)]
pub struct GetReservationsPerSalesOrder{
    #[validate(length(min =1, message = "Order Number is required"))]
    pub order_number: String,
    #[validate(length(min =1, message = "Product Code is required"))]
    pub product_code: String,
    #[validate(length(min =1, message = "Color is required"))]
    pub color: String,
    #[validate(length(min =1, message = "Product Name is required"))]
    pub product_name: String,
    #[validate(range(min =1, max=10000, message = "Pcs are required and must be between 1 and 10000"))]
    pub order_pcs: i32,
    #[validate(length(min =1, message = "Company Name is required"))]
    pub company: String,
    #[validate(range(min =0, max=10000, message = "Pcs are required and must be between 1 and 10000"))]
    pub reserved_pcs: i32,
    pub fulfilment_perc: f32
}

impl FromRow for GetReservationsPerSalesOrder {
    fn from_row(row: Row) -> Self{

        let (order_number, product_code,color,  product_name, order_pcs,company,  reserved_pcs, fulfilment_perc):(String, String, String, String, i32, String,i32, f32) = mysql_async::from_row(row);

        GetReservationsPerSalesOrder{order_number, product_code,color,  product_name, order_pcs,company,  reserved_pcs, fulfilment_perc}
    }

    fn from_row_opt(row: Row) -> Result<Self, mysql_async::FromRowError> {
        let (order_number, product_code,color,  product_name, order_pcs,company,  reserved_pcs, fulfilment_perc):(String, String, String, String, i32, String,i32, f32) = mysql_async::from_row(row);

        Ok(GetReservationsPerSalesOrder{order_number, product_code,color,  product_name, order_pcs,company,  reserved_pcs, fulfilment_perc})
    }

}

#[derive(Validate, Deserialize, Serialize, Debug)]
pub struct DeleteReservations{
    #[validate(length(min =1, message = "Order Number is required"))]
    pub order_number: String
}