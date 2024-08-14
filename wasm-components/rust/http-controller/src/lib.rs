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
        let method = request.method();
        let path_and_query = request.path_with_query().unwrap();

        log(
            Level::Info,
            "http-controller",
            format!("Received {:?} request at {}", method, path_and_query).as_str(),
        );

        let (path_parts, query) = parse_path_and_query(&path_and_query);

        match path_parts.as_slice() {
            ["products", path_parts @ ..] => {
                Routes::products(path_parts, query, request, response_out)
            }
            ["data-init", path_parts @ ..] => {
                Routes::data_init(path_parts, query, request, response_out)
            }
            ["inventory", path_parts @ ..] => {
                Routes::inventory(path_parts, query, request, response_out)
            }
            ["orders", path_parts @ ..] => Routes::orders(path_parts, query, request, response_out),
            _ => response_out.complete_response(404, b"404 Not Found\n"),
        }
    }
}

fn parse_path_and_query(path: &str) -> (Vec<&str>, Option<&str>) {
    let (path, query) = path.split_at(path.find('?').unwrap_or(path.len()));
    let query: Option<&str> = if query.is_empty() {
        None
    } else {
        Some(query.trim_start_matches("?"))
    };

    let path_parts: Vec<&str> = path
        .strip_prefix('/')
        .map(|remainder| remainder.split('/'))
        .map(|c| c.collect())
        .unwrap_or_default();
    (path_parts, query)
}

struct Routes;

// TODO: improve error handling everywhere
// TODO: refactor this into less of a mess
impl Routes {
    fn products(
        path_parts: &[&str],
        _query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        let method = request.method();

        if !path_parts.is_empty() {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        match method {
            Method::Get => {
                let products =
                    list_products().expect("HTTP-CONTROLLER-PRODUCTS-GET: failed to list products");
                let product_data = products
                    .iter()
                    .map(|product| ProductData::from(product.clone()))
                    .collect::<Vec<ProductData>>();
                let products_json = json!(product_data).to_string();

                response_out.complete_response(200, products_json.as_bytes());
            }
            Method::Post => {
                let body = request
                    .read_body()
                    .expect("HTTP-CONTROLLER-PRODUCTS-POST: failed to read body");
                let product: Product = serde_json::from_slice::<ProductData>(&body).unwrap().into();
                create_product(&product)
                    .expect("HTTP-CONTROLLER-PRODUCTS-POST: failed to create product");
                response_out.complete_response(201, "Created".as_bytes());
            }
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
        }
    }

    fn data_init(
        path_parts: &[&str],
        _query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        let method = request.method();

        if path_parts.len() > 1 {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        match method {
            Method::Get => match path_parts {
                ["all"] => {
                    init_all()
                        .expect("HTTP-CONTROLLER-DATA-INIT-ALL failed to initialize products");
                    response_out.complete_response(
                        200,
                        "Products, inventory and orders schema initialized".as_bytes(),
                    )
                }
                ["products"] => {
                    init_products().expect(
                        "HTTP-CONTROLLER-DATA-INIT-PRODUCTS: failed to initialize products",
                    );
                    response_out.complete_response(200, "Products initialized".as_bytes())
                }
                ["inventory"] => {
                    init_inventory().expect(
                        "HTTP-CONTROLLER-DATA-INIT-INVENTORY: failed to initialize inventory",
                    );
                    response_out.complete_response(200, "Inventory initialized".as_bytes())
                }
                ["orders"] => {
                    init_orders().expect(
                        "HTTP-CONTROLLER-DATA-INIT-ORDERS: failed to initialize orders schema",
                    );
                    response_out.complete_response(200, "Orders schema initialized".as_bytes())
                }
                _ => response_out.complete_response(404, b"404 Not Found\n"),
            },
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
        }
    }

    fn inventory(
        path_parts: &[&str],
        query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        if !path_parts.is_empty() {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        if query.is_none() {
            return response_out.complete_response(400, b"400 Bad Request\n");
        }

        if let Some(value) = query {
            if !value.contains("skus=") {
                return response_out.complete_response(400, b"400 Bad Request\n");
            }
        }

        let method = request.method();

        match method {
            Method::Get => {
                let query_str = query.unwrap();
                let mut query_pairs = form_urlencoded::parse(query_str.as_bytes());

                let skus_string = query_pairs.find(|(key, _)| key == "skus").unwrap().1;

                let skus_list: Vec<String> =
                    skus_string.split(',').map(|s| s.to_string()).collect();

                let inventory = get_inventory(skus_list.as_slice())
                    .expect("HTTP-CONTROLLER-INVENTORY-GET: failed to fetch inventory");
                let inventory_data: Vec<AvailabilityData> = inventory
                    .iter()
                    .map(|entry| AvailabilityData::from(entry.clone()))
                    .collect();

                let inventory_json = json!(inventory_data).to_string();

                response_out.complete_response(200, inventory_json.as_bytes())
            }
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
        }
    }

    fn orders(
        path_parts: &[&str],
        _query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        if !path_parts.is_empty() {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        let method = request.method();

        match method {
            Method::Get => {
                let orders =
                    get_orders().expect("HTTP-CONTROLLER-ORDERS-GET: failed to get orders");
                let order_data: Vec<OrderData> = orders
                    .iter()
                    .map(|order| OrderData::from(order.clone()))
                    .collect();
                let orders_json = json!(order_data).to_string();

                response_out.complete_response(200, orders_json.as_bytes())
            }
            Method::Post => {
                let body = request
                    .read_body()
                    .expect("HTTP-CONTROLLER-ORDERS-POST: failed to read body");
                let line_item_data: Vec<common::orders::LineItem> =
                    serde_json::from_slice(&body).unwrap();

                let line_items: Vec<LineItem> = line_item_data.iter().map(LineItem::from).collect();

                let create_response = create_order(line_items.as_slice());

                match create_response {
                    Ok(_) => response_out.complete_response(201, "Created".as_bytes()),
                    Err(e) => {
                        let OrderError::Internal(msg) = e;
                        response_out.complete_response(
                            500,
                            format!("Unable to place order: {}", msg).as_bytes(),
                        )
                    }
                }
            }
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
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
