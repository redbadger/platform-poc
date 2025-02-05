wit_bindgen::generate!({
    world: "platform-poc:inventory-service/inventory-service",
    generate_all,
});

use exports::platform_poc::inventory::inventory::Guest;
use platform_poc::inventory::types::{Availability, Error};
use wasi::logging::logging::{log, Level};
use wasmcloud::postgres::{
    query::{query, PgValue},
    types::ResultRowEntry,
};

struct Component;
export!(Component);

impl Guest for Component {
    fn get_inventory(skus: Vec<String>) -> Result<Vec<Availability>, Error> {
        log(Level::Info, "inventory-service", "Getting inventory...");

        let mut inventory: Vec<Availability> = Vec::new();

        for sku in skus {
            let sql_result = query(
                "SELECT sku, quantity FROM inventory.t_inventory WHERE sku = $1",
                &[PgValue::Text(sku)],
            )
            .expect("INVENTORY-SERVICE-GET-INVENTORY: failed to query inventory");
            // TODO: making an assumption that there's always at most 1 entry, this is not enforced by the database
            // schema - would be nice to handle this better
            if !sql_result.is_empty() {
                let row = &sql_result[0];

                let availability = row.iter().fold(
                    Availability::default(),
                    |mut acc: Availability, entry: &ResultRowEntry| {
                        match entry.column_name.as_str() {
                            "sku" => {
                                acc.sku = if let PgValue::Text(sku) = &entry.value {
                                    sku.to_string()
                                } else {
                                    "".to_string()
                                };
                            }
                            "quantity" => {
                                acc.is_in_stock = if let PgValue::Int4(quantity) = entry.value {
                                    quantity > 0
                                } else {
                                    false
                                };
                            }
                            _ => {}
                        }
                        acc
                    },
                );

                inventory.push(availability);
            }
        }

        Ok(inventory)
    }
}

impl Default for Availability {
    fn default() -> Self {
        Availability {
            sku: Default::default(),
            is_in_stock: Default::default(),
        }
    }
}
