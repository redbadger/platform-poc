wit_bindgen::generate!({
    world: "inventory-service",
    exports: {
        "platform-poc:inventory/inventory": InventoryService,
    }
});

use exports::platform_poc::inventory::inventory::{Availability, Guest as InventoryExport};

struct InventoryService;

impl InventoryExport for InventoryService {
    fn get_inventory(sku_code: Vec<i32>) -> Vec<Availability> {
        // FIXME read from an SQL database
        sku_code
            .iter()
            .map(|sku| Availability {
                sku_code: *sku,
                is_in_stock: true,
            })
            .collect()
    }
}
