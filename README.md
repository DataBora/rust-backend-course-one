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

I only made public 2 end points for you to try it out:
GET /unique_identifiers
Which retrieves all the data form unique_identifiers table(warehouse table)
example for Postamn:
http://localhost:8080/unique_identifiers

POST /add_or_update_unique_identifier
This endpint firstly goes to check if certain product exists in the specific location, if yes just adds the pcs to existing value, if not is creates a new row in the table.
example for Postman:
{
"color": "Wenge",
"product_name" : "Amora Set",
"warehouse": "HALA 3",
"location": "M2-C-33",
"pcs" : 77
}
