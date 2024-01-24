use mysql_async::{prelude::Queryable, Error, Value};
use crate::models::incoming::UniqueIdentifier;
use crate::models::incoming::AddOrUpdateUniqueIdentifierRequest;
use crate::models::outgoing::RemoveUniqueIdentifierRequest;

#[derive(Clone)]
pub struct Database {
    pub pool: mysql_async::Pool,
}

impl Database {
    pub async fn init() -> Result<Self, mysql_async::Error> {
        let db_url = "mysql://user:pass@127.0.0.1:3306/wms";
        let pool = mysql_async::Pool::new(db_url);
        Ok(Database { pool })
    }

    //functions for unique_identifiers

    pub async fn get_all_locations(&self) ->  Result<Vec<UniqueIdentifier>, Error> {
        let query = "SELECT * FROM unique_identifiers";
        let mut conn = self.pool.get_conn().await.unwrap();

        let locations: Vec<UniqueIdentifier> = conn.query(query).await.unwrap();

        Ok(locations)
    }

    pub async fn add_or_update_unique_identifier(&self, update_data: &AddOrUpdateUniqueIdentifierRequest) -> Result<(), mysql_async::Error> {
        // Build the concatenated string based on the update data
        let update_concatenated_string = format!(
            "{}^{}^{}^{}",
            update_data.color, update_data.product_name, update_data.warehouse, update_data.location
        );
    
        // Insert or update the row using the MySQL syntax
        let query = "INSERT INTO unique_identifiers (concatenated_string, color, product_name, warehouse, location, pcs) VALUES (?, ?, ?, ?, ?, ?)
                     ON DUPLICATE KEY UPDATE pcs = pcs + VALUES(pcs)";
        let params: Vec<_> = vec![
            Value::from(&update_concatenated_string),
            Value::from(&update_data.color),
            Value::from(&update_data.product_name),
            Value::from(&update_data.warehouse),
            Value::from(&update_data.location),
            Value::from(update_data.pcs),
        ];
    
        let mut conn = self.pool.get_conn().await?;
        conn.exec_drop(query, params).await?;
    
        Ok(())
    }

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
    



