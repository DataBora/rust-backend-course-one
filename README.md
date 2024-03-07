# Warehouse Management System (WMS)

This project is to show how I solved real world implementation for Warehouse Management.

Full WMS project that I implemented for my client: Production Company - Mega Plast Jovanovic d.o.o. Nova Pazova, Serbia, evolves around 5 tables, and 5 modules: Warehouse, Sales Orders, Operations Planing, Reservations, and Assembling.

This implementation doesn't require scanning the products after productin, and before entering the Warehouse, but if needed, it's easy adoption.

This real life WMS is covering 2 warehouse with over 3000 locations, and it is used by 2 warehouse managers, without any collisions.

My front end is build within Excel from which all the API's are called.
You can check YouTube video to see live presentation: https://www.youtube.com/watch?v=F2kNQLhCr-E

Backend is implemented with Rust and SQL.
Fronted with VBA and Excel.

### Prerequisites

- Rust 1.75
- MySQL 80
- Excel (Office 365)

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

6. **GET /sales_orders**

Thsi end-point retrieves all data from all sales orders.

7. **GET /sales_orders/{order_number}**

This end-point retrieves the data from the sales orders by order number.

8. **POST /insert_sales_order**

This end-point takes Sales Order Excel file, from local file path, parse the file and enters all the rows into database.

9. **DELETE /delete_sales_order/{order_number}**

This end-point deletes sales order by order number.

10. **POST /add_reservation**

This end-point adds products on reservation per order number.

11. **DELETE /delete_reservation/{order_number}**

This end-point deletes reservations related to order number.

12. **GET /get_order_fulfilment/{order_number}**

This end-point retrieves % of order fullfilment per order number.

13. **GET /sales_order_products_operations/{order_number}**

This end-point compares warehouse stock levels and checks difference between stock levels and sales order pcs.
Data is ordered in ascending so that stock is taken from lowest to highest stock level locations.
