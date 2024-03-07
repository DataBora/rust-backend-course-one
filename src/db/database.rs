use std::env;
use dotenv::dotenv;

use mysql_async::{prelude::Queryable, Error, Value, params};
use crate::models::outgoing::RemoveUniqueIdentifierRequest;
use crate::models::incoming::{GetProductLocationsByCode,GetProductLocationsByName, UniqueIdentifier,AddOrUpdateUniqueIdentifierRequest};
use crate::models::salesorder::{SalesOrder,GetSalesOrder,SalesOrderProduct};
use crate::models::reservations::{AddReservationForOrderNumber,GetReservationsPerSalesOrder,DeleteReservations};


#[derive(Clone)]
pub struct Database {
    pub pool: mysql_async::Pool,
}

impl Database {
    pub async fn init() -> Result<Self, mysql_async::Error> {
        dotenv().ok();
        let db_url = env::var("MYSQL_DB_URL").expect("MYSQL_DB_URL not set in .env file");
        let pool = mysql_async::Pool::new(db_url.as_str());
        Ok(Database { pool })
    }
    // -------------- DATABASE FUNCTIONS ------------------ //
    //functions for unique_identifiers to get all locations

    pub async fn get_all_locations(&self) ->  Result<Vec<UniqueIdentifier>, Error> {
        let query = "SELECT * FROM unique_identifiers";
        let mut conn = self.pool.get_conn().await.unwrap();

        let locations: Vec<UniqueIdentifier> = conn.query(query).await.unwrap();

        Ok(locations)
    }

    //  get locations for single product by PRODUCT NAME
    pub async fn get_product_locations_by_name(&self, product: &GetProductLocationsByName) -> Result<Vec<UniqueIdentifier>, Error> {

        let product_name = &product.product_name;
        
        let query = "SELECT * FROM unique_identifiers WHERE product_name = :product_name";
     
        let named_params = params! {
            "product_name" => product_name,
        };
      
        let mut conn = self.pool.get_conn().await?;

        let locations: Vec<UniqueIdentifier> = conn.exec(query, named_params.clone()).await?;

        Ok(locations)
    }

    // get locations for single product by PRODUCT CODE
    pub async fn get_product_locations_by_code(&self, product: &GetProductLocationsByCode) -> Result<Vec<UniqueIdentifier>, Error> {

        let product_code = &product.product_code;
        
        let query = "SELECT * FROM unique_identifiers WHERE product_code = :product_code";
     
        let named_params = params! {
            "product_code" => product_code,
        };
      
        let mut conn = self.pool.get_conn().await?;

        let locations: Vec<UniqueIdentifier> = conn.exec(query, named_params.clone()).await?;

        Ok(locations)
    }


      //functions for sales_orders to get all orders
      pub async fn get_sales_orders(&self) ->  Result<Vec<SalesOrder>, Error> {
        let query = "SELECT * FROM sales_orders";
        let mut conn = self.pool.get_conn().await.unwrap();

        let locations: Vec<SalesOrder> = conn.query(query).await.unwrap();

        Ok(locations)
    }

    // function for unique identifiers to get locations for single product
    pub async fn get_sales_order_by_po(&self, order: &GetSalesOrder) -> Result<Vec<SalesOrder>, Error> {

        let order_number = &order.order_number;
        
        let query = "SELECT * FROM sales_orders WHERE order_number = :order_number";
     
        let named_params = params! {
            "order_number" => order_number,
        };
      
        let mut conn = self.pool.get_conn().await?;

        let orders: Vec<SalesOrder> = conn.exec(query, named_params.clone()).await?;

        Ok(orders)
    }

    
    //adding or updationg existing row in the database
    pub async fn add_or_update_unique_identifier(&self, update_data: &AddOrUpdateUniqueIdentifierRequest) -> Result<(), mysql_async::Error> {
        // Extract information from update_data
        let color = &update_data.color;
        let product_name = &update_data.product_name;
    
        // Query products table to get product_code
        let query_product = "SELECT product_code FROM products WHERE color = :color AND product_name = :product_name";
        let params_product = params! {
            "color" => color,
            "product_name" => product_name,
        };
    
        let mut conn = self.pool.get_conn().await?;
        let product_code: Option<String> = conn.exec_first(query_product, params_product).await?;
    
        // Build the concatenated string based on the update data
        let update_concatenated_string = format!(
            "{}^{}^{}^{}",
            update_data.color, update_data.product_name, update_data.warehouse, update_data.location
        );
    
        // Insert or update the row using the MySQL
        let query_unique_identifier = "INSERT INTO unique_identifiers (concatenated_string, product_code, color, product_name, warehouse, location, pcs) VALUES (?, ?, ?, ?, ?, ?, ?)
                                        ON DUPLICATE KEY UPDATE pcs = pcs + VALUES(pcs)";
        let params_unique_identifier: Vec<_> = vec![
            Value::from(&update_concatenated_string),
            Value::from(&product_code),
            Value::from(&update_data.color),
            Value::from(&update_data.product_name),
            Value::from(&update_data.warehouse),
            Value::from(&update_data.location),
            Value::from(update_data.pcs),
        ];
    
        conn.exec_drop(query_unique_identifier, params_unique_identifier).await?;
    
        Ok(())
    }

    //removing values from ocs column or removing row from database if pcs = 0
    pub async fn remove_unique_identifier(&self, update_data: &RemoveUniqueIdentifierRequest) -> Result<(), mysql_async::Error> {
        // Build concatenated string based on the update data
        let update_concatenated_string = format!(
            "{}^{}^{}^{}",
            update_data.color, update_data.product_name, update_data.warehouse, update_data.location
        );
    
        // Check if the user's requested pcs is greater than the current value in the database
        let check_current_pcs_query = "SELECT pcs FROM unique_identifiers WHERE concatenated_string = ?";
        let check_current_pcs_params: Vec<Value> = vec![Value::from(&update_concatenated_string)];
        let current_pcs: Option<i32> = self.pool.get_conn().await?.exec_first(check_current_pcs_query, check_current_pcs_params).await?;
    
        if let Some(current_pcs_value) = current_pcs {
            if update_data.pcs > current_pcs_value {
                // Respond with an error indicating that there's not enough pcs for deduction
                return Err(mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not enough pcs for deduction")));
            }
        } else {
            // Handle the case where the concatenated_string is not found in the database
            return Err(mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Concatenated string not found")));
        }
    
        // Update pcs field
        let update_query = "UPDATE unique_identifiers SET pcs = pcs - ? WHERE concatenated_string = ?";
        let update_params: Vec<_> = vec![
            Value::from(update_data.pcs),
            Value::from(&update_concatenated_string),
        ];
        let mut conn = self.pool.get_conn().await?;
        conn.exec_drop(update_query, update_params).await?;
    
        // Check if updated pcs is less than or equal to 0, and delete the row if necessary
        let check_updated_pcs_query = "SELECT pcs FROM unique_identifiers WHERE concatenated_string = ?";
        let check_updated_pcs_params: Vec<Value> = vec![Value::from(&update_concatenated_string)];
        let updated_pcs: Option<i32> = conn.exec_first(check_updated_pcs_query, check_updated_pcs_params).await?;
    
        if let Some(pcs) = updated_pcs {
            if pcs <= 0 {
                let delete_query = "DELETE FROM unique_identifiers WHERE concatenated_string = ?";
                let delete_params: Vec<Value> = vec![Value::from(&update_concatenated_string)];
                conn.exec_drop(delete_query, delete_params).await?;
            }
        }
    
        Ok(())
    }
    
 
    //inserting sales order in the database
    pub async fn insert_sales_order(&self, sales_orders: Vec<SalesOrder>) -> Result<(), mysql_async::Error> {
    
        let query = r#"
            INSERT INTO sales_orders (order_number, product_code, color, product_name, pcs, company)
            VALUES (:order_number, :product_code, :color, :product_name, :pcs, :company)
        "#;

    
        let mut conn = self.pool.get_conn().await?;

        let mut transaction = conn.start_transaction(mysql_async::TxOpts::default()).await?;

        // Iterate over each sales order and execute the insert query
        for sales_order in sales_orders {
            // Prepare parameters for the query
            let params = params! {
                "order_number" => &sales_order.order_number,
                "product_code" => &sales_order.product_code,
                "color" => &sales_order.color,
                "product_name" => &sales_order.product_name,
                "pcs" => &sales_order.pcs,
                "company"=> &sales_order.company,
            };
            transaction.exec_drop::<&str, mysql_async::Params>(query.as_ref(), params).await?;
            // Execute the query
            
        }
        transaction.commit().await?;


        Ok(())
    }

        // Check if sales order exists
    pub async fn check_sales_order_existence(&self, po_number: &GetSalesOrder) -> Result<bool, Error> {
        let order_number = &po_number.order_number;
        let query = "SELECT COUNT(*) FROM sales_orders WHERE order_number = :order_number";
        let named_params = params! {
            "order_number" => order_number,
        };

        let mut conn = self.pool.get_conn().await?;
        let count: u64 = conn.exec_first(query, named_params.clone()).await?.unwrap();
        
        Ok(count > 0)
    }

    //delete sales order by order number
    pub async fn delete_sales_order(&self, po_number: &GetSalesOrder)-> Result<(),Error> {
        
        let order_number = &po_number.order_number;
      
        let query = "DELETE FROM sales_orders WHERE order_number = :order_number";
     
        let named_params = params! {
            "order_number" => order_number,
        };
      
        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(query, named_params.clone()).await?;

        Ok(())

    }

      //check if reservation exists
      pub async fn check_reservations_existance(&self, po_number: &DeleteReservations)->Result<bool, Error>{
        let order_number = &po_number.order_number;

        let query = "SELECT COUNT(*) FROM reservations where order_number = :order_number";

        let named_params = params! {
            "order_number" => order_number,
        };

        let mut conn = self.pool.get_conn().await?;
        let count: u64 = conn.exec_first(query, named_params.clone()).await?.unwrap();
        Ok(count>0)


    }

    // delete reservation by order number
    pub async fn delete_reservation(&self, po_number: &DeleteReservations)-> Result<(),Error> {
        
        let order_number = &po_number.order_number;
      
        let query = "DELETE FROM reservations WHERE order_number = :order_number";
     
        let named_params = params! {
            "order_number" => order_number,
        };
      
        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(query, named_params.clone()).await?;

        Ok(())

    }


    //GET for comparing sales orders and unique identifiers 
    pub async fn get_sales_order_products_operations(
       &self,
        po_number: &GetSalesOrder,
    ) -> Result<Vec<SalesOrderProduct>, mysql_async::Error> {

        let order_number = &po_number.order_number;

        let query = r#"
            WITH ranked_locations AS (
                SELECT
                    u.product_code,
                    u.color,
                    u.product_name,
                    u.warehouse,
                    u.location,
                    u.pcs AS warehouse_pcs,
                    COALESCE(s.pcs, 0) AS order_pcs,
                    ROW_NUMBER() OVER (PARTITION BY u.product_code ORDER BY u.pcs) AS location_rank
                FROM unique_identifiers u
                LEFT JOIN sales_orders s ON u.product_code = s.product_code AND s.order_number = :order_number
            )
            SELECT
                product_code,
                color,
                product_name,
                warehouse,
                location,
                warehouse_pcs,
                order_pcs,
                CASE
                    WHEN order_pcs <= 0 THEN 0  -- No order to fulfill
                    WHEN SUM(warehouse_pcs) OVER (PARTITION BY product_code ORDER BY location_rank) >= order_pcs THEN 
                        GREATEST(order_pcs - COALESCE(SUM(warehouse_pcs) OVER (PARTITION BY product_code ORDER BY location_rank ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING), 0), 0) 
                    ELSE 
                        GREATEST(warehouse_pcs, 0)  -- Fulfill the order from this location
                END AS deducted_pcs,
                CASE
                    WHEN order_pcs <= 0 THEN 0  -- No order to fulfill
                    WHEN SUM(warehouse_pcs) OVER (PARTITION BY product_code ORDER BY location_rank) >= order_pcs THEN 
                        GREATEST(warehouse_pcs - (order_pcs - COALESCE(SUM(warehouse_pcs) OVER (PARTITION BY product_code ORDER BY location_rank ROWS BETWEEN UNBOUNDED PRECEDING AND 1 PRECEDING), 0)), 0)
                    ELSE 
                        warehouse_pcs - order_pcs 
                END AS difference
            FROM ranked_locations;
        "#;
        
        let named_params = params! {
            "order_number" => order_number,
        };

        let mut conn = self.pool.get_conn().await?;

       let sales_order_product =  conn.exec(query, named_params.clone()).await?;
       Ok(sales_order_product)
    }


    pub async fn add_reservation(&self, reservation: &AddReservationForOrderNumber) -> Result<(), mysql_async::Error> {
       
        let order_number = &reservation.order_number;
        let product_code = &reservation.product_code;
        let current_warehouse = &reservation.current_warehouse;
        let current_location = &reservation.current_location;
        let reservation_warehouse = &reservation.reservation_warehouse;
        let reservation_location = &reservation.reservation_location;
        let pcs = &reservation.pcs;
    
        let query_current = r#"
        SELECT pcs
        FROM unique_identifiers
        WHERE product_code = :product_code AND warehouse = :current_warehouse AND location = :current_location
         "#;
    
   
        let mut conn = self.pool.get_conn().await?;
      
         let query_params = params! {
            "product_code" => product_code,
            "current_warehouse" => current_warehouse,
            "current_location" => current_location,
        };
    
   
        let current_pcs: Option<i32> = conn.exec_first(query_current, query_params).await?;

        // Ensure the product exists
        let current_pcs = current_pcs.ok_or_else(|| mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")))?;
    
        // Check if requested pcs is greater than the current value in the database
        if *pcs > current_pcs {
            return Err(mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Not enough pcs for deduction")));
        }
//----------------------------------------------------------------------------------------
        let query_current_sales = r#"
            SELECT pcs
            FROM sales_orders
            WHERE order_number = :order_number AND product_code = :product_code
         "#;
    
   
        let mut conn = self.pool.get_conn().await?;
      
        let query_params_sales = params! {
            "order_number" => order_number,
            "product_code" => product_code
        };

        let sales_order_pcs: Option<i32> = conn.exec_first(query_current_sales, query_params_sales).await?;

          // Ensure the product exists
        let sales_order_pcs = sales_order_pcs.ok_or_else(|| mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")))?;

         // Check if requested pcs is greater than the sales_order pcs
        if *pcs > sales_order_pcs{
            return Err(mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "That is more Pcs than neccessary for Order Fulfilment.")));
        }
    //------------------------------------------------------------------------------------
        let query_reserved_pcs = r#"
        SELECT 
        CASE 
            WHEN EXISTS (
                SELECT pcs
                FROM reservations
                WHERE order_number = :order_number AND product_code = :product_code
            )
            THEN (SELECT pcs FROM reservations WHERE order_number = :order_number AND product_code = :product_code)
            ELSE 0 
        END AS reserved_pcs;
        "#;


        let mut conn = self.pool.get_conn().await?;
    
        let query_params_reserved_pcs = params! {
            "order_number" => order_number,
            "product_code" => product_code
        };

        let reserved_pcs: Option<i32> = conn.exec_first(query_reserved_pcs, query_params_reserved_pcs).await?;

        // Ensure the product exists
        let reserved_pcs = reserved_pcs.ok_or_else(|| mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Product not found")))?;

         // Check if requested pcs is greater than the sales_order pcs + 
         if (*pcs + reserved_pcs) > sales_order_pcs{
            return Err(mysql_async::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, "That is more Pcs than neccessary for Order Fulfilment.")));
        }


    //------------------------------------------------------------------------------------
      
        // Update the remaining quantity in the unique_identifiers table
        let query_update = r#"
            UPDATE unique_identifiers
            SET pcs = pcs - :pcs
            WHERE product_code = :product_code AND warehouse = :current_warehouse AND location = :current_location
        "#;

        // Execute the update query
        conn.exec_drop(query_update, params! {
            "pcs" => pcs,
            "product_code" => product_code,
            "current_warehouse" => current_warehouse,
            "current_location" => current_location,
        }).await?;

          // Check if the updated quantity is 0, and delete the row if necessary
          let delete_query = r#"
          DELETE FROM unique_identifiers
          WHERE product_code = :product_code AND warehouse = :current_warehouse AND location = :current_location AND pcs <= 0
      "#;

      conn.exec_drop(delete_query, params! {
          "product_code" => product_code,
          "current_warehouse" => current_warehouse,
          "current_location" => current_location,
      }).await?;


        // Check if the combination of order_number and product_code is unique
        let duplicate_check_params = params! {
            "order_number" => order_number,
            "product_code" => product_code,
        };
        let duplicate_check_query = r#"
            SELECT COUNT(*) AS count
            FROM reservations
            WHERE order_number = :order_number AND product_code = :product_code
        "#;

        let duplicate_count: Option<i64> = conn.exec_first(duplicate_check_query, duplicate_check_params).await?;

        // If duplicate_count > 0, the combination is not unique, perform an update
        if let Some(count) = duplicate_count {
            if count > 0 {
                // Update the existing reservation
                let update_query = r#"
                    UPDATE reservations
                    SET pcs = pcs + :pcs
                    WHERE order_number = :order_number AND product_code = :product_code
                "#;

                conn.exec_drop(update_query, params! {
                    "order_number" => order_number,
                    "product_code" => product_code,
                    "pcs" => pcs,
                }).await?;
                return Ok(());
            }
        }

        // If the combination is unique, perform an insertion
        let insert_query = r#"
            INSERT INTO reservations (order_number, product_code, reservation_warehouse, reservation_location, pcs)
            VALUES (:order_number, :product_code, :reservation_warehouse, :reservation_location, :pcs)
        "#;

        conn.exec_drop(insert_query, params! {
            "order_number" => order_number,
            "product_code" => product_code,
            "reservation_warehouse" => reservation_warehouse,
            "reservation_location" => reservation_location,
            "pcs" => pcs,
        }).await?;

      
        Ok(())
    }

    pub async fn get_order_fulfilment(&self, order_no: &GetSalesOrder)-> Result<Vec<GetReservationsPerSalesOrder>, mysql_async::Error>{

        let order_number = &order_no.order_number;

        let query = r#"
                SELECT 
                so.order_number,
                so.product_code,
                so.color,
                so.product_name,
                so.pcs AS order_pcs,
                so.company,
                COALESCE(r.pcs, 0) AS reserved_pcs,
                CASE
                    WHEN so.pcs > 0 THEN ROUND((COALESCE(r.pcs, 0) / so.pcs) * 100, 2)
                    ELSE 0
                END AS fulfilment_perc
            FROM
                sales_orders so
            LEFT JOIN
                reservations r ON so.order_number = r.order_number
                AND so.product_code = r.product_code
            WHERE
                so.order_number = :order_number;
                "#;

    
      // Define the parameters
        let query_params = params! {
            "order_number" => order_number,
        };
      
        let mut conn = self.pool.get_conn().await?;

        let fulfilment: Vec<GetReservationsPerSalesOrder> = conn.exec(query, query_params).await?;

        Ok(fulfilment)
       


    }
    
    
}



// --------------- TESTING ----------------- //
    
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_all_locations() {
        // Arrange: Initialize the connection pool
        let db = Database::init().await.unwrap();
        // Act: Call the function you want to test
        let result = db.get_all_locations().await;
        // Assert: Check if the result is as expected
        match result {
            Ok(locations)=> {
                assert!(!locations.is_empty())
            }
            Err(err)=>{
                panic!("Error {:?}", err)
            }

        }


    }
}


