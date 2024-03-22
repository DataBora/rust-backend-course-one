use actix_web::{HttpServer, App, web::Data};

mod db;
mod models;
mod api;

use crate::db::database::Database;

use api::mysqlapi::{get_unique_identifiers, get_locations_by_product_code, get_locations_by_product_name, add_or_update_unique_identifier,  remove_unique_identifier};



#[actix_web::main]
async fn main()-> std::io::Result<()> {
    match Database::init().await {
        Ok(db) => {
            println!("Database initialized successfully");
            let db_data = Data::new(db);

            HttpServer::new(move||{
                App::new()
                    .app_data(db_data.clone())
                    .service(get_unique_identifiers)
                    .service(get_locations_by_product_code)
                    .service(get_locations_by_product_name)
                    .service(add_or_update_unique_identifier)
                    .service(remove_unique_identifier)
                    
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        }
        Err(err) => {
            eprintln!("Error connecting to the database: {}", err);
            std::process::exit(1);
        }
    }

}
