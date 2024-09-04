wit_bindgen::generate!({
    world: "platform-poc:di/di",
    path: [
        "../../wit/data-init",
        "../../wit/deps/wasmcloud/postgres",
        "../../wit/deps/wasi/keyvalue",
        "../../wit/deps/wasi/logging",
        "wit",
    ],
    generate_all,
});

use uuid::Uuid;

use common::products::Product;
use exports::platform_poc::data_init::init_funcs::Guest;
use wasi::{
    keyvalue::store::open,
    logging::logging::{log, Level},
};
use wasmcloud::postgres::query::{query, PgValue};

struct Component;

impl Guest for Component {
    fn init_all() -> Result<(), String> {
        Component::init_products().expect("DATA-INIT-ALL: failed to initialize products");
        Component::init_inventory().expect("DATA-INIT-ALL: failed to initialize inventory");
        Component::init_orders().expect("DATA-INIT-ALL: failed to initialize orders");
        Ok(())
    }

    fn init_products() -> Result<(), String> {
        let bucket = open("").expect("DATA-INIT-PRODUCTS: failed to open bucket");
        for product in sample_products() {
            let product_json = serde_json::to_string(&product)
                .expect("DATA-INIT-PRODUCTS: failed to convert product to json");
            bucket
                .set(&product.sku, product_json.as_bytes())
                .expect("DATA-INIT-PRODUCTS: failed to set product");
        }
        log(Level::Info, "data-init", "Products initialized!");
        Ok(())
    }

    fn init_inventory() -> Result<(), String> {
        query(
            "-- Create the schema if it does not exist
        CREATE SCHEMA IF NOT EXISTS inventory;
        ",
            &[],
        )
        .expect("DATA-INIT-INVENTORY: failed to create inventory namespace");

        query(
            "-- Create the table in the inventory schema only if it does not exist
        CREATE TABLE IF NOT EXISTS inventory.t_inventory (
            id bigint NOT NULL GENERATED BY DEFAULT AS IDENTITY,
            quantity integer,
            sku text,
            PRIMARY KEY (id)
        );",
            &[],
        )
        .expect("DATA-INIT-INVENTORY: failed to create inventory table");

        let products = sample_products();

        let (available, unavailable) = products.split_at(products.len() / 2);

        for available_product in available {
            query(
                "INSERT INTO inventory.t_inventory (quantity, sku) VALUES ($1, $2);",
                &[
                    PgValue::Integer(10),
                    PgValue::Text(available_product.sku.clone()),
                ],
            )
            .expect("DATA-INIT-INVENTORY: failed to insert inventory");
        }
        for unavailable_product in unavailable {
            query(
                "INSERT INTO inventory.t_inventory (quantity, sku) VALUES ($1, $2);",
                &[
                    PgValue::Integer(0),
                    PgValue::Text(unavailable_product.sku.clone()),
                ],
            )
            .expect("DATA-INIT-INVENTORY: failed to insert inventory");
        }
        Ok(())
    }

    fn init_orders() -> Result<(), String> {
        query("CREATE SCHEMA IF NOT EXISTS orders;", &[])
            .expect("DATA-INIT-ORDERS: failed to create orders schema");

        query(
            "-- Table: orders.t_orders
        CREATE TABLE IF NOT EXISTS orders.t_orders (
            id bigint NOT NULL GENERATED BY DEFAULT AS IDENTITY,
            order_number text,
            total integer,
            PRIMARY KEY (id)
        );",
            &[],
        )
        .expect("DATA-INIT-ORDERS: failed to create orders table");

        query(
            "-- Table: orders.t_order_line_items
        CREATE TABLE IF NOT EXISTS orders.t_order_line_items (
            id bigint NOT NULL GENERATED BY DEFAULT AS IDENTITY,
            price integer,
            quantity integer,
            sku text,
            PRIMARY KEY (id)
        );",
            &[],
        )
        .expect("DATA-INIT-ORDERS: failed to create order line items table");

        query(
            "-- Table: orders.t_orders_order_line_items_list
        CREATE TABLE IF NOT EXISTS orders.t_orders_order_line_items_list (
            order_id bigint NOT NULL,
            order_line_items_list_id bigint NOT NULL,
            UNIQUE (order_line_items_list_id),
            FOREIGN KEY (order_line_items_list_id)
                REFERENCES orders.t_order_line_items (id),
            FOREIGN KEY (order_id)
                REFERENCES orders.t_orders (id)
        );",
            &[],
        )
        .expect("DATA-INIT-ORDERS: failed to create orders order line items list table");

        Ok(())
    }
}

fn sample_products() -> Vec<Product> {
    vec![
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Car Engine"),
            description: String::from("V8 engine with 500 horsepower"),
            price: 8500,
            sku: String::from("ENG-V8-500"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Brake Pads"),
            description: String::from("High performance brake pads"),
            price: 150,
            sku: String::from("BRK-PD-HP"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Air Filter"),
            description: String::from("Premium air filter for increased airflow"),
            price: 30,
            sku: String::from("AIR-FLT-PREM"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Spark Plugs"),
            description: String::from("High-efficiency spark plugs"),
            price: 60,
            sku: String::from("SPK-PLG-HI-EFF"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Tire Set"),
            description: String::from("Set of 4 all-season tires"),
            price: 600,
            sku: String::from("TIR-SET-AS"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Battery"),
            description: String::from("High-capacity car battery"),
            price: 120,
            sku: String::from("BAT-HC-12V"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Windshield Wipers"),
            description: String::from("All-weather windshield wipers"),
            price: 45,
            sku: String::from("WND-WPR-AW"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Fuel Pump"),
            description: String::from("Electric fuel pump for efficient fuel delivery"),
            price: 220,
            sku: String::from("FL-PMP-ELEC"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Radiator"),
            description: String::from("High-efficiency car radiator"),
            price: 320,
            sku: String::from("RAD-HI-EFF"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Headlights"),
            description: String::from("LED headlights with long lifespan"),
            price: 250,
            sku: String::from("HDL-LED-LONG"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Alternator"),
            description: String::from("High output alternator for enhanced performance"),
            price: 300,
            sku: String::from("ALT-HO-ENH"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Exhaust System"),
            description: String::from("Performance exhaust system"),
            price: 750,
            sku: String::from("EXH-SYS-PERF"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Suspension Kit"),
            description: String::from("Complete suspension kit for improved handling"),
            price: 900,
            sku: String::from("SUS-KIT-IMP"),
        },
        Product {
            id: Uuid::new_v4().to_string(),
            name: String::from("Turbocharger"),
            description: String::from("High-performance turbocharger"),
            price: 1400,
            sku: String::from("TRB-CHR-HP"),
        },
    ]
}

export!(Component);
