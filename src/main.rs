use actix_web::{HttpServer, App, web::Data};

mod db;
mod models;
mod api;

use crate::db::database::Database;

use api::mysqlapi::{get_unique_identifiers, get_locations_for_single_product_by_code, get_locations_for_single_product_by_name,get_order_fulfilment, get_sales_order_by_po, get_sales_orders, get_sales_order_products_operations, add_or_update_unique_identifier, add_reservation, insert_sales_order, delete_reservation, delete_sales_order, remove_unique_identifier};



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
                    .service(get_locations_for_single_product_by_code)
                    .service(get_locations_for_single_product_by_name)
                    .service(add_or_update_unique_identifier)
                    .service(remove_unique_identifier)
                    .service(insert_sales_order)
                    .service(get_sales_orders)
                    .service(get_sales_order_by_po)
                    .service(delete_sales_order)
                    .service(delete_reservation)
                    .service(get_sales_order_products_operations)
                    .service(add_reservation)
                    .service(get_order_fulfilment)
                    
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
