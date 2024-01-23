use mysql_async::{prelude::Queryable, Error, Value};
use crate::models::wposition::UniqueIdentifier;
use crate::models::wposition::AddOrUpdateUniqueIdentifierRequest;

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
    
    
}
    



