wit_bindgen::generate!({
    world: "inventory-service",
    exports: {
        "platform-poc:inventory/inventory": InventoryService,
    }
});

use exports::platform_poc::inventory::inventory::Guest as InventoryExport;

struct InventoryService;

impl InventoryExport for InventoryService {
    fn hello() -> String {
        "hello from inventory".to_string()
    }
}
