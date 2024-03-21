# RUST BACKEND COURSE

This course is to show how I solved the part of real world implementation for Warehouse Management.

Full WMS project that I implemented for my client: Production Company - Mega Plast Jovanovic d.o.o. Nova Pazova, Serbia, evolves around 5 tables, and 5 modules: Warehouse, Sales Orders, Operations Planing, Reservations, and Assembling.

This implementation doesn't require scanning the products after production, and before entering the Warehouse, but if needed, it's easy adoption.

This real life WMS is covering 2 warehouse with over 3000 locations, and it is used by 2 warehouse managers, without any collisions.

### Prerequisites

- Rust 1.77
- MySQL 80
- Postman

### Dependencies

- Actix Web
- actix-web
- async-trait
- derive_more
- json
- mysql_async
- serde
- serde_json
- validator
- tokio
- dotenv

### API Endpoints

1. **GET /unique_identifiers**

This end-point retrieves all the data from the `unique_identifiers` table (warehouse table).

2. **GET /unique_identifiers_name/{product_name}**

This end-point retrieves all the data from the `unique_identifiers` by product name.

3. **GET /unique_identifiers_code/{product_code}**

This end-point retrieves all data from the `unique_identifiers` table (warehouse table) by product code.

4. **POST /add_or_update_unique_identifier**

This end-point first checks if a certain product exists at the specific location. If yes, it adds the specified quantity (pcs) to the existing value. If not, it creates a new row in the table.

5. **DELETE /remove_unique_identifiers**

This end-point deducts the required quantity from a certain row in unique_identifier table. Firstly checks if the row exists by unique identifier, then verifies if the quantity for pcs inserted is greater than the value in the database. If these conditions pass, the deduction takes place. If the value in the row of the identified product reaches 0, the row gets deleted from the table.
