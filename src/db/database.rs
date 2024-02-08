
use mysql_async::{prelude::Queryable, Error, Value, params};
use crate::models::incoming::UniqueIdentifier;
use crate::models::incoming::AddOrUpdateUniqueIdentifierRequest;
use crate::models::outgoing::RemoveUniqueIdentifierRequest;
use crate::models::incoming::GetProductLocations;
use crate::models::incoming::GetProductLocationsByCode;
use crate::models::reservations::DeleteReservations;
use crate::models::salesorder::SalesOrder;
use crate::models::salesorder::GetSalesOrder;
use crate::models::salesorder::SalesOrderProduct;
use crate::models::reservations::AddReservationForOrderNumber;
use crate::models::reservations::GetReservationsPerSalesOrder;




#[derive(Clone)]
pub struct Database {
    pub pool: mysql_async::Pool,
}

impl Database {
    pub async fn init() -> Result<Self, mysql_async::Error> {
        let db_url = "mysql://databora:!Djavolak1@127.0.0.1:3306/wms";
        let pool = mysql_async::Pool::new(db_url);
        Ok(Database { pool })
    }

    //functions for unique_identifiers to get all locations

    pub async fn get_all_locations(&self) ->  Result<Vec<UniqueIdentifier>, Error> {
        let query = "SELECT * FROM unique_identifiers";
        let mut conn = self.pool.get_conn().await.unwrap();

        let locations: Vec<UniqueIdentifier> = conn.query(query).await.unwrap();

        Ok(locations)
    }

    //  get locations for single product by PRODUCT NAME
    pub async fn get_product_locations(&self, product: &GetProductLocations) -> Result<Vec<UniqueIdentifier>, Error> {

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

    //delete reservation by order number
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
                    SELECT
                    u.product_code,
                    u.color,
                    u.product_name,
                    u.warehouse,
                    u.location,
                    COALESCE(u.pcs, 0) AS warehouse_pcs,
                    COALESCE(s.pcs, 0) AS order_pcs,
                    COALESCE(u.pcs, 0) - COALESCE(s.pcs, 0) AS pcs_difference
                FROM sales_orders s
                LEFT JOIN unique_identifiers u ON s.product_code = u.product_code
                WHERE s.order_number = :order_number
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
    



