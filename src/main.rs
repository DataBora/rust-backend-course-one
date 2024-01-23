use actix_web::web::Data;
use actix_web::{get, post,  Responder, HttpResponse, HttpServer, App, web::Json};
mod models;
mod db;
use crate::db::Database;
use crate::models::wposition::AddOrUpdateUniqueIdentifierRequest;
use validator::Validate;

//GET / unique identifiers
#[get("/unique_identifiers")]
async fn get_unique_identifiers(db: Data<Database>) -> impl Responder{
  
    match db.get_all_locations().await {
        Ok(found_locations) => HttpResponse::Ok().json(found_locations),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving unique identifiers"),
    }
}


//POST /unique_identifiers
#[post("/add_or_update_unique_identifier")]
async fn add_or_update_unique_identifier(body: Json<AddOrUpdateUniqueIdentifierRequest>, db: Data<Database>) -> impl Responder {
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
                    .service(add_or_update_unique_identifier)
                    
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
