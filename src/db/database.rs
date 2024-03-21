use std::env;
use dotenv::dotenv;

use mysql_async::{prelude::Queryable, Error, Value, params};
use crate::models::outgoing::RemoveUniqueIdentifierRequest;
use crate::models::incoming::{GetProductLocationsByCode,GetProductLocationsByName, UniqueIdentifier,AddOrUpdateUniqueIdentifierRequest};


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


