wit_bindgen::generate!({
    world: "platform-poc:http-controller/http-controller",
    path: [
        "../../wit/deps/wasi/io",
        "../../wit/deps/wasi/random",
        "../../wit/deps/wasi/clocks",
        "../../wit/deps/wasi/filesystem",
        "../../wit/deps/wasi/sockets",
        "../../wit/deps/wasi/cli",
        "../../wit/deps/wasi/http",
        "../../wit/deps/wasi/logging",
        "../../wit/inventory",
        "../../wit/orders",
        "../../wit/data-init",
        "../../wit/products",
        "wit",
    ],
    generate_all,
});

use routefinder::Router;
use waki::{handler, ErrorCode, Method, Request, Response};
use wasi::logging::logging::{log, Level};

use common::{
    inventory::Availability as AvailabilityData,
    orders::{LineItem as LineItemData, Order as OrderData},
    products::Product as ProductData,
};
use platform_poc::{
    data_init::init_funcs::{init_all, init_inventory, init_orders, init_products},
    inventory::inventory::{get_inventory, Availability},
    orders::orders::{create_order, get_orders, Error as OrderError, LineItem, Order},
    products::products::{create_product, list_products, Product},
};

const MODULE: &str = "http-controller";

#[handler]
fn handler(request: Request) -> Result<Response, ErrorCode> {
    log(
        Level::Info,
        MODULE,
        &format!(
            "Received {:?} request at {}{:?}",
            request.method(),
            request.path(),
            request.query()
        ),
    );

    let mut router = Router::new();

    router
        .add("/data-init/:action", Handlers::DataInit)
        .expect("adding route");
    router
        .add("/inventory", Handlers::Inventory)
        .expect("adding route");
    router
        .add("/orders", Handlers::Orders)
        .expect("adding route");
    router
        .add("/products", Handlers::Products)
        .expect("adding route");

    let Some(m) = router.best_match(request.path()) else {
        return response::not_found();
    };

    match m.handler() {
        Handlers::DataInit => {
            let captures = m.captures();
            let Ok(schema) = captures.get("action").try_into() else {
                return response::bad_request();
            };
            Handlers::data_init(schema, request)
        }
        Handlers::Inventory => Handlers::inventory(request),
        Handlers::Orders => Handlers::orders(request),
        Handlers::Products => Handlers::products(request),
    }
}

enum Schema {
    All,
    Inventory,
    Orders,
    Products,
}

impl TryFrom<Option<&str>> for Schema {
    type Error = ();

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some("all") => Ok(Self::All),
            Some("inventory") => Ok(Self::Inventory),
            Some("orders") => Ok(Self::Orders),
            Some("products") => Ok(Self::Products),
            _ => Err(()),
        }
    }
}

enum Handlers {
    DataInit,
    Inventory,
    Orders,
    Products,
}

impl Handlers {
    fn data_init(schema: Schema, request: Request) -> Result<Response, ErrorCode> {
        match request.method() {
            Method::Get => match schema {
                Schema::All => match init_all() {
                    Ok(()) => response::ok(),
                    Err(e) => {
                        response::server_error(&format!("failed to initialize all schemas: {e}"))
                    }
                },
                Schema::Inventory => match init_inventory() {
                    Ok(()) => response::ok(),
                    Err(e) => response::server_error(&format!(
                        "failed to initialize inventory schema: {e}"
                    )),
                },
                Schema::Orders => match init_orders() {
                    Ok(()) => response::ok(),
                    Err(e) => {
                        response::server_error(&format!("failed to initialize orders schema: {e}"))
                    }
                },
                Schema::Products => match init_products() {
                    Ok(()) => response::ok(),
                    Err(e) => response::server_error(&format!(
                        "failed to initialize products schema: {e}"
                    )),
                },
            },
            _ => response::method_not_allowed(),
        }
    }

    fn inventory(request: Request) -> Result<Response, ErrorCode> {
        const KEY: &str = "skus";
        let query = request.query();
        if query.is_empty() || !query.contains_key(KEY) {
            return response::bad_request();
        }

        match request.method() {
            Method::Get => {
                let skus = &query[KEY];
                let skus: Vec<String> = skus.split(',').map(|s| s.to_string()).collect();

                match get_inventory(&skus) {
                    Ok(inventory) => {
                        let body: Vec<AvailabilityData> =
                            inventory.iter().map(Into::into).collect();
                        response::ok_with_json(&body)
                    }
                    Err(e) => response::server_error(&format!("failed to get inventory: {e}")),
                }
            }
            _ => response::method_not_allowed(),
        }
    }

    fn orders(request: Request) -> Result<Response, ErrorCode> {
        match request.method() {
            Method::Get => match get_orders() {
                Ok(orders) => {
                    let body: Vec<OrderData> = orders.iter().map(Into::into).collect();
                    response::ok_with_json(&body)
                }
                Err(e) => response::server_error(&format!("failed to get orders: {e}")),
            },
            Method::Post => {
                let Ok(items) = request.json::<Vec<common::orders::LineItem>>() else {
                    return response::bad_request();
                };

                let items: Vec<LineItem> = items.iter().map(Into::into).collect();

                match create_order(&items) {
                    Ok(()) => response::created(),
                    Err(OrderError::Internal(msg)) => {
                        response::server_error(&format!("failed to create order: {msg}"))
                    }
                }
            }
            _ => response::method_not_allowed(),
        }
    }

    fn products(request: Request) -> Result<Response, ErrorCode> {
        match request.method() {
            Method::Get => match list_products() {
                Ok(products) => {
                    let body: Vec<ProductData> = products.iter().map(Into::into).collect();
                    response::ok_with_json(&body)
                }
                Err(e) => response::server_error(&format!("failed to list products: {e}")),
            },
            Method::Post => {
                let Ok(data) = request.json::<ProductData>().as_ref().map(Into::into) else {
                    return response::bad_request();
                };

                match create_product(&data) {
                    Ok(()) => response::created(),
                    Err(e) => response::server_error(&format!("failed to create product: {e}")),
                }
            }
            _ => response::method_not_allowed(),
        }
    }
}

mod response {
    use crate::{
        wasi::logging::logging::{log, Level},
        MODULE,
    };
    use waki::{ErrorCode, Response};

    pub fn ok() -> Result<Response, ErrorCode> {
        Response::builder().status_code(200).body("200 OK").build()
    }

    pub fn ok_with_json<T: serde::Serialize>(data: &T) -> Result<Response, ErrorCode> {
        Response::builder().status_code(200).json(data).build()
    }

    pub fn created() -> Result<Response, ErrorCode> {
        Response::builder()
            .status_code(201)
            .body("201 Created")
            .build()
    }

    pub fn not_found() -> Result<Response, ErrorCode> {
        Response::builder()
            .status_code(404)
            .body("404 Not Found")
            .build()
    }

    pub fn method_not_allowed() -> Result<Response, ErrorCode> {
        Response::builder()
            .status_code(405)
            .body("405 Method Not Allowed")
            .build()
    }

    pub fn bad_request() -> Result<Response, ErrorCode> {
        Response::builder()
            .status_code(400)
            .body("400 Bad Request")
            .build()
    }

    pub fn server_error(msg: &str) -> Result<Response, ErrorCode> {
        log(Level::Error, MODULE, msg);
        Response::builder()
            .status_code(500)
            .body("500 Internal Server Error")
            .build()
    }
}

impl From<&Product> for ProductData {
    fn from(product: &Product) -> Self {
        ProductData {
            id: product.id.clone(),
            name: product.name.clone(),
            description: product.description.clone(),
            price: product.price,
            sku: product.sku.clone(),
        }
    }
}

impl From<&ProductData> for Product {
    fn from(product: &ProductData) -> Self {
        Product {
            id: product.id.clone(),
            name: product.name.clone(),
            description: product.description.clone(),
            price: product.price,
            sku: product.sku.clone(),
        }
    }
}

impl From<&Availability> for AvailabilityData {
    fn from(product: &Availability) -> Self {
        AvailabilityData {
            sku: product.sku.clone(),
            is_in_stock: product.is_in_stock,
        }
    }
}

impl From<&LineItemData> for LineItem {
    fn from(value: &LineItemData) -> Self {
        LineItem {
            sku: value.sku.clone(),
            price: value.price,
            quantity: value.quantity,
        }
    }
}

impl From<&LineItem> for LineItemData {
    fn from(value: &LineItem) -> Self {
        LineItemData {
            sku: value.sku.clone(),
            price: value.price,
            quantity: value.quantity,
        }
    }
}

impl From<&Order> for OrderData {
    fn from(order: &Order) -> Self {
        OrderData {
            order_number: order.order_number.clone(),
            total: order.total,
            line_items: order.line_items.iter().map(Into::into).collect(),
        }
    }
}
