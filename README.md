# Warehouse Management System (WMS) API

This repository is to show how solved real world implementation for Warehouse Management System, for Incoming products module.

My real WMS that I implemented for my client Production Company - Mega Plast Jovanovic d.o.o., evolves around 1 single table, and 6 functions, 1 for each module.
All the data for products, warehouses, locations, colors are coming from remote data list validatons that user just inputs into designated fields with easy search by typing into fields.
Same can be achieved with scanner, only input changes.

This real world wms is covering 2 warehouse with over 3000 locations, and is used by 2 warehouse managers.

My front end is build in Excel from witch I call all of the API's.

Simple as it gets, and cheap as it gets, and it works perfectly with no collisiions as users never work on the same rows.

This repository contains the backend API for a Warehouse Management System implemented in Rust.

### Prerequisites

- Rust 1.75
- MySQL 80

### Installation

1. Clone the repository:

   ```bash
   git clone git@github.com:DataBora/wms-api-rust.git
   ```

### API Endpoints

### API Endpoints

I have implemented 3 public endpoints for you to try:

1. **GET /unique_identifiers**

   Retrieves all data from the `unique_identifiers` table (warehouse table).

   Example for Postman:

   ```plaintext
   http://localhost:8080/unique_identifiers
   ```

POST /add_or_update_unique_identifier

This endpoint first checks if a certain product exists at the specific location. If yes, it adds the specified quantity (pcs) to the existing value. If not, it creates a new row in the table.

Example for Postman:

````json
{
  "color": "Wenge",
  "product_name": "Amora Set",
  "warehouse": "HALA 3",
  "location": "M2-C-33",
  "pcs": 77
}


DELETE /remove_unique_identifiers

This endpoint deducts the required quantity from a certain row. It checks if the row exists by unique identifier, then verifies if the quantity for pcs inserted is greater than the value in the database. If these conditions pass, the deduction takes place. If the value in the row of the identified product reaches 0, the row gets deleted from the table.

Example for Postman:

```json
{
  "color": "Wenge",
  "product_name": "Amora Set",
  "warehouse": "HALA 3",
  "location": "M2-C-33",
  "pcs": 77
}
````
