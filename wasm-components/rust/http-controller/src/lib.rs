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

use anyhow::{anyhow, Result};
use routefinder::Router;
use serde_json::json;

use common::{
    inventory::Availability as AvailabilityData,
    orders::{LineItem as LineItemData, Order as OrderData},
    products::Product as ProductData,
};
use exports::wasi::http::incoming_handler::Guest;
use platform_poc::{
    data_init::init_funcs::{init_all, init_inventory, init_orders, init_products},
    inventory::inventory::{get_inventory, Availability},
    orders::orders::{create_order, get_orders, Error as OrderError, LineItem, Order},
    products::products::{create_product, list_products, Product},
};
use wasi::{
    http::types::{Method, *},
    logging::logging::{log, Level},
};

mod http;

struct Component;
export!(Component);

impl Guest for Component {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        match handle(request) {
            Ok((status_code, body)) => {
                response_out.complete_response(status_code, body.as_bytes());
            }
            Err(e) => {
                log(
                    Level::Error,
                    "http-controller",
                    format!("Error: {:?}", e).as_str(),
                );
                response_out.complete_response(500, b"Internal Server Error");
            }
        };
    }
}

fn handle(request: IncomingRequest) -> Result<(StatusCode, String)> {
    let method = request.method();
    let path_with_query = request.path_with_query().unwrap_or_default();
    let (path, query) = http::path_and_query(&path_with_query);

    log(
        Level::Info,
        "http-controller",
        format!("Received {:?} request at {}", method, path_with_query).as_str(),
    );

    let mut router = Router::new();

    router
        .add("/data-init/:action", Handlers::DataInit)
        .map_err(|e| anyhow!("adding route: {}", e))?;
    router
        .add("/inventory", Handlers::Inventory)
        .map_err(|e| anyhow!("adding route: {}", e))?;
    router
        .add("/orders", Handlers::Orders)
        .map_err(|e| anyhow!("adding route: {}", e))?;
    router
        .add("/products", Handlers::Products)
        .map_err(|e| anyhow!("adding route: {}", e))?;

    let Some(m) = router.best_match(path) else {
        return Ok((404, "404 Not Found\n".to_string()));
    };

    match m.handler() {
        Handlers::DataInit => {
            let captures = m.captures();
            let action = captures.get("action").unwrap_or_default();
            Handlers::data_init(action, request)
        }
        Handlers::Inventory => Handlers::inventory(query, request),
        Handlers::Orders => Handlers::orders(request),
        Handlers::Products => Handlers::products(request),
    }
}

enum Handlers {
    DataInit,
    Inventory,
    Orders,
    Products,
}

impl Handlers {
    fn products(request: IncomingRequest) -> Result<(StatusCode, String)> {
        match request.method() {
            Method::Get => {
                let products = list_products().map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-PRODUCTS-GET: failed to list products: {e}")
                })?;
                let product_data = products
                    .iter()
                    .map(|product| ProductData::from(product.clone()))
                    .collect::<Vec<ProductData>>();
                Ok((200, json!(product_data).to_string()))
            }
            Method::Post => {
                let body = request.read_body().map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-PRODUCTS-POST: failed to read body: {e}")
                })?;
                let Ok(data) = serde_json::from_slice::<ProductData>(&body) else {
                    return Ok((400, "400 Bad Request\n".to_string()));
                };
                let product: Product = data.into();
                create_product(&product).map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-PRODUCTS-POST: failed to create product: {e}")
                })?;
                Ok((201, "201 Created\n".to_string()))
            }
            _ => Ok((405, "405 Method Not Allowed\n".to_string())),
        }
    }

    fn data_init(action: &str, request: IncomingRequest) -> Result<(StatusCode, String)> {
        match request.method() {
            Method::Get => match action {
                "all" => {
                    init_all().map_err(|e| {
                        anyhow!("HTTP-CONTROLLER-DATA-INIT-ALL failed to initialize products: {e}")
                    })?;
                    Ok((
                        200,
                        "Products, inventory and orders schema initialized\n".to_string(),
                    ))
                }
                "products" => {
                    init_products().map_err(|e| {
                        anyhow!(
                        "HTTP-CONTROLLER-DATA-INIT-PRODUCTS: failed to initialize products: {e}")
                    })?;
                    Ok((200, "Products initialized\n".to_string()))
                }
                "inventory" => {
                    init_inventory().map_err(|e| {
                        anyhow!("HTTP-CONTROLLER-DATA-INIT-INVENTORY: failed to initialize inventory: {e}")
                    })?;
                    Ok((200, "Inventory initialized\n".to_string()))
                }
                "orders" => {
                    init_orders().map_err(|e| {
                        anyhow!("HTTP-CONTROLLER-DATA-INIT-ORDERS: failed to initialize orders schema: {e}")
                    })?;
                    Ok((200, "Orders schema initialized\n".to_string()))
                }
                _ => Ok((404, "404 Not Found\n".to_string())),
            },
            _ => Ok((405, "405 Method Not Allowed\n".to_string())),
        }
    }

    fn inventory(query: Option<&str>, request: IncomingRequest) -> Result<(StatusCode, String)> {
        if query.is_none() {
            return Ok((400, "400 Bad Request\n".to_string()));
        }

        if let Some(value) = query {
            if !value.contains("skus=") {
                return Ok((400, "400 Bad Request\n".to_string()));
            }
        }

        match request.method() {
            Method::Get => {
                let query_str = query.unwrap_or_default();
                let mut query_pairs = form_urlencoded::parse(query_str.as_bytes());

                let skus_string = query_pairs
                    .find(|(key, _)| key == "skus")
                    .map(|(_, v)| v.to_string())
                    .unwrap_or_default();

                let skus_list: Vec<String> =
                    skus_string.split(',').map(|s| s.to_string()).collect();

                let inventory = get_inventory(skus_list.as_slice()).map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-INVENTORY-GET: failed to fetch inventory: {e}")
                })?;
                let inventory_data: Vec<AvailabilityData> = inventory
                    .iter()
                    .map(|entry| AvailabilityData::from(entry.clone()))
                    .collect();

                Ok((200, json!(inventory_data).to_string()))
            }
            _ => Ok((405, "405 Method Not Allowed\n".to_string())),
        }
    }

    fn orders(request: IncomingRequest) -> Result<(StatusCode, String)> {
        match request.method() {
            Method::Get => {
                let orders = get_orders().map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-ORDERS-GET: failed to get orders: {e}")
                })?;
                let order_data: Vec<OrderData> = orders
                    .iter()
                    .map(|order| OrderData::from(order.clone()))
                    .collect();
                Ok((200, json!(order_data).to_string()))
            }
            Method::Post => {
                let body = request.read_body().map_err(|e| {
                    anyhow!("HTTP-CONTROLLER-ORDERS-POST: failed to read body: {e}")
                })?;
                let Ok(data) = serde_json::from_slice::<Vec<common::orders::LineItem>>(&body)
                else {
                    return Ok((400, "400 Bad Request\n".to_string()));
                };

                let line_items: Vec<LineItem> = data.iter().map(LineItem::from).collect();

                let create_response = create_order(line_items.as_slice());

                match create_response {
                    Ok(_) => Ok((201, "201 Created\n".to_string())),
                    Err(e) => {
                        let OrderError::Internal(msg) = e;
                        Ok((500, format!("Unable to place order: {}\n", msg)))
                    }
                }
            }
            _ => Ok((405, "405 Method Not Allowed\n".to_string())),
        }
    }
}

impl From<Product> for ProductData {
    fn from(product: Product) -> Self {
        ProductData {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            sku: product.sku,
        }
    }
}

impl From<ProductData> for Product {
    fn from(product: ProductData) -> Self {
        Product {
            id: product.id,
            name: product.name,
            description: product.description,
            price: product.price,
            sku: product.sku,
        }
    }
}

impl From<Availability> for AvailabilityData {
    fn from(product: Availability) -> Self {
        AvailabilityData {
            sku: product.sku,
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

impl From<LineItem> for LineItemData {
    fn from(value: LineItem) -> Self {
        LineItemData {
            sku: value.sku.clone(),
            price: value.price,
            quantity: value.quantity,
        }
    }
}

impl From<Order> for OrderData {
    fn from(order: Order) -> Self {
        OrderData {
            order_number: order.order_number.clone(),
            total: order.total,
            line_items: order
                .line_items
                .iter()
                .map(|line_item| LineItemData::from(line_item.clone()))
                .collect(),
        }
    }
}
