
use actix_web::web::{Data, Path};
use actix_web::{get, post, Responder, HttpResponse, HttpServer, App, web::Json,delete};
use models::outgoing::RemoveUniqueIdentifierRequest;

mod models;
mod db;

use crate::db::Database;
use crate::models::incoming::AddOrUpdateUniqueIdentifierRequest;
use crate::models::incoming::GetProductLocations;
use crate::models::salesorder::SalesOrder;
use crate::models::salesorder::GetSalesOrder;
use crate::models::reservations::AddReservationForOrderNumber;

use validator::Validate;

//GET / unique identifiers
#[get("/unique_identifiers")]
async fn get_unique_identifiers(db: Data<Database>) -> impl Responder {
    match db.get_all_locations().await {
        Ok(found_locations) => {
            if found_locations.is_empty() {
                HttpResponse::NotFound().body("No data available in the database")
            } else {
                HttpResponse::Ok().json(found_locations)
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
    }
}

//GET / unique identifier locations for single product
#[get("/unique_identifiers/{product_name}")]
async fn get_locations_for_single_product( db: Data<Database>, product: Path<GetProductLocations>) -> impl Responder {
    
    let is_valid = product.validate(); 

    match is_valid {
        Ok(_) => {
            match db.get_product_locations(&product).await {
                Ok(locations) => {
                    if !locations.is_empty() {
                        HttpResponse::Ok().json(locations)
                    } else {
                        HttpResponse::NotFound().body("No locations found for the specified product.")
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to find the locations."),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
    }

  
}

//GET / sales orders
#[get("/sales_orders")]
async fn get_sales_orders(db: Data<Database>) -> impl Responder {
    match db.get_sales_orders().await {
        Ok(found_orders) => {
            if found_orders.is_empty() {
                HttpResponse::NotFound().body("No data available in the database")
            } else {
                HttpResponse::Ok().json(found_orders)
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving Sales Orders"),
    }
}

//GET / sales orders by order number
#[get("/sales_orders/{order_number}")]
async fn get_sales_order_by_po( db: Data<Database>, order: Path<GetSalesOrder>) -> impl Responder {
    
    let is_valid = order.validate(); 

    match is_valid {
        Ok(_) => {
            match db.get_sales_order_by_po(&order).await {
                Ok(orders) => {
                    if !orders.is_empty() {
                        HttpResponse::Ok().json(orders)
                    } else {
                        HttpResponse::NotFound().body("No Sales Orders found for the specified PO number.")
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to find the Sales Orders."),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving Sales Orders"),
    }

}


//POST /unique_identifiers
#[post("/add_or_update_unique_identifier")]
async fn add_or_update_unique_identifier( db: Data<Database>, body: Json<AddOrUpdateUniqueIdentifierRequest>) -> impl Responder {
    
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            match db.add_or_update_unique_identifier(&body).await {
                Ok(_) => HttpResponse::Ok().body("Identifier added or updated successfully!"),
                Err(_) => HttpResponse::InternalServerError().body("Failed to add or update identifier"),
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Invalid input. Please provide valid identifier details."),
    }
}

//inserting sales order into database by parsing excel file
#[post("/insert_sales_order")]
async fn insert_sales_order(
    db: Data<Database>,
    payload: Json<Vec<SalesOrder>>,
) -> HttpResponse {
    // Validate the received JSON data
    let sales_orders = payload.into_inner();
    let is_valid: bool = sales_orders.iter().all(|order| order.validate().is_ok());

    if is_valid {
        // Insert valid sales orders into the database
        match db.insert_sales_order(sales_orders).await {
            Ok(_) => HttpResponse::Ok().body("Sales orders added successfully!"),
            Err(err) => HttpResponse::InternalServerError()
                .body(format!("Error inserting sales orders into the database: {}", err)),
        }
    } else {
        HttpResponse::BadRequest().body("Validation failed for one or more sales orders.")
    }
}

//DELETE sales order
#[delete("delete_sales_order/{order_number}")]
async fn delete_sales_order(db: Data<Database>, po_number: Path<GetSalesOrder>) -> impl Responder {

    let is_valid = po_number.validate();

    match is_valid {
        Ok(_) => {
            match db.delete_sales_order(&po_number).await {
                Ok(_) => HttpResponse::Ok().body("Sales Order deleted successfully!"),
                Err(_) => HttpResponse::NotFound().body("No Sales Orders found for the specified PO number."),
                }
                
            },
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving Sales Orders"),
    }

  
}


//UPDATE or DELETE unique identifiers
#[delete("/remove_unique_identifiers")]
async fn remove_unique_identifier(body: Json<RemoveUniqueIdentifierRequest>, db: Data<Database>) -> impl Responder {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
        match db.remove_unique_identifier(&body).await{
            Ok(_)=> HttpResponse::Ok().body("Identifier updated or removed succefully!"),
            Err(_)=> HttpResponse::InternalServerError().body("Faile to update or remove identifier. Posible reason: Not enough quantity for removal."),
            }
        }
        Err(_)=> HttpResponse::BadRequest().body("Invalid input. Please provide valid identifier details.")
    }
}

//GET sales order product difference with unique identifiers
#[get("/sales_order_products_operations/{order_number}")]
async fn get_sales_order_products_operations(db: Data<Database>, order_number: Path<GetSalesOrder>) -> impl Responder {

    let is_valid = &order_number.validate();

    match is_valid{
        Ok(_)=>{
            match db.get_sales_order_products_operations(&order_number).await{
                Ok(products)=>HttpResponse::Ok().json(products),
                Err(_) => HttpResponse::InternalServerError().body("Failed")
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to retreive data from database.")
    }

}

#[post("/add_reservation")]
async fn add_reservation(
    db: Data<Database>,
    body: Json<AddReservationForOrderNumber>,
) -> HttpResponse {
    // Validate the received JSON data
    let reservations = body.into_inner();
    let is_valid: bool = reservations.validate().is_ok();

    if is_valid {
        // Insert valid sales orders into the database
        match db.add_reservation(&reservations).await {
            Ok(_) => HttpResponse::Ok().body("Reservation added successfully!"),
            Err(err) => HttpResponse::InternalServerError()
                .body(format!("Error inserting reservation into the database: {}", err)),
        }
    } else {
        HttpResponse::BadRequest().body("Validation failed for one or more reservation value.")
    }
}

#[get("/get_order_fulfilment/{order_number}")]
async fn get_order_fulfilment(db: Data<Database>, order_number: Path<GetSalesOrder>) -> impl Responder {

    let is_valid = &order_number.validate();

    match is_valid{
        Ok(_)=>{
            match db.get_order_fulfilment(&order_number).await{
                Ok(order)=>HttpResponse::Ok().json(order),
                Err(_) => HttpResponse::InternalServerError().body("Failed to find requested order")
            }
        }
        Err(_) => HttpResponse::BadRequest().body("Failed to retreive data from database.")
    }

}

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
                    .service(get_locations_for_single_product)
                    .service(add_or_update_unique_identifier)
                    .service(remove_unique_identifier)
                    .service(insert_sales_order)
                    .service(get_sales_orders)
                    .service(get_sales_order_by_po)
                    .service(delete_sales_order)
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
