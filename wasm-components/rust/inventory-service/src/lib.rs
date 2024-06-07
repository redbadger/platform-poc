wit_bindgen::generate!({
    world: "inventory-service"
});

use common::inventory::Availability as AvailabilityData;
use exports::platform_poc::inventory::inventory::Guest;
use platform_poc::inventory::types::{Availability, Error};
use wasmcloud::postgres::query::query;
use wasmcloud::postgres::query::PgValue;
use wasmcloud::postgres::types::ResultRowEntry;

use wasi::logging::logging::{log, Level};

struct HttpServer;

impl Into<Availability> for AvailabilityData {
    fn into(self) -> Availability {
        Availability {
            sku: self.sku,
            is_in_stock: self.is_in_stock,
        }
    }
}

impl Guest for HttpServer {
    fn get_inventory(skus: Vec<String>) -> Result<Vec<Availability>, Error> {
        log(Level::Info, "inventory-service", "Getting inventory...");
        
        let mut inventory: Vec<Availability> = Vec::new();

        for sku in skus {
            let sql_result = query(
                "SELECT sku, quantity FROM inventory.t_inventory WHERE sku = $1",
                &[PgValue::Text(sku)],
            ).expect("INVENTORY-SERVICE-GET-INVENTORY: failed to query inventory");
            // TODO: making an assumption that there's always at most 1 entry, this is not enforced by the database
            // schema - would be nice to handle this better
            if !sql_result.is_empty() {
                let row = &sql_result[0];

                let availability_data = row.iter().fold(AvailabilityData::default(), |mut acc: AvailabilityData , entry: &ResultRowEntry| {
                    match entry.column_name.as_str() {
                        "sku" => {
                            acc.sku = if let PgValue::Text(sku) = &entry.value {
                                sku.to_string()
                            } else {
                                "".to_string()
                            };
                        },
                        "quantity" => {
                            acc.is_in_stock = if let PgValue::Int4(quantity) = entry.value {
                                quantity > 0
                            } else {
                                false
                            };
                        },
                        _ => {}
                    }
                    acc
                });
                
                inventory.push(availability_data.into());
            }
        }

        Ok(inventory)
    }
}

export!(HttpServer);
