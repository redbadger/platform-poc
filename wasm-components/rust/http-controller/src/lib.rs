wit_bindgen::generate!({
    world: "http-controller"
});

use anyhow::{anyhow, bail, Result};
use common::inventory::Availability as AvailabilityData;
use common::products::Product as ProductData;
use exports::wasi::http::incoming_handler::Guest;
use platform_poc::data_init::init_funcs::{init_all, init_inventory, init_orders, init_products};
use platform_poc::inventory::inventory::{get_inventory, Availability};
use platform_poc::products::products::{create_product, list_products, Product};
use serde_json::json;
use url::Url;
use wasi::http::types::Method;
use wasi::http::types::*;
use wasi::io::streams::StreamError;
use wasi::logging::logging::{log, Level};

const MAX_READ_BYTES: u32 = 2048;

struct HttpServer;

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

impl Into<Product> for ProductData {
    fn into(self) -> Product {
        Product {
            id: self.id,
            name: self.name,
            description: self.description,
            price: self.price,
            sku: self.sku,
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

impl Into<Availability> for AvailabilityData {
    fn into(self) -> Availability {
        Availability {
            sku: self.sku,
            is_in_stock: self.is_in_stock,
        }
    }
}

// TODO: imprtove general error handling everywhere
impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let path = request.path_with_query().unwrap();
        let method = request.method();

        log(
            Level::Info,
            "http-controller",
            format!("Received {:?} request at {}", method, path).as_str(),
        );

        let parsed_url =
            Url::parse(&format!("http://example.com{}", path)).expect("Failed to parse URL");

        let path_parts: Vec<&str> = parsed_url
            .path_segments()
            .map(|c| c.map(|c| c).collect())
            .unwrap_or_else(Vec::new);

        match path_parts.as_slice() {
            ["products", path_rest @ ..] => {
                Routes::products(path_rest, parsed_url.query(), request, response_out)
            }
            ["data-init", path_rest @ ..] => {
                Routes::data_init(path_rest, parsed_url.query(), request, response_out)
            }
            ["inventory", path_rest @ ..] => {
                Routes::inventory(path_rest, parsed_url.query(), request, response_out)
            }
            _ => response_out.complete_response(404, b"404 Not Found\n"),
        }
    }
}

impl ResponseOutparam {
    fn complete_response(self, status_code: StatusCode, body: &[u8]) {
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(status_code).unwrap();
        let response_body = response.body().unwrap();
        ResponseOutparam::set(self, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(body)
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

impl IncomingRequest {
    fn read_body(self) -> Result<Vec<u8>> {
        let incoming_req_body = self
            .consume()
            .map_err(|()| anyhow!("failed to consume incoming request body"))?;
        let incoming_req_body_stream = incoming_req_body
            .stream()
            .map_err(|()| anyhow!("failed to build stream for incoming request body"))?;
        let mut buf = Vec::<u8>::with_capacity(MAX_READ_BYTES as usize);
        loop {
            match incoming_req_body_stream.blocking_read(MAX_READ_BYTES as u64) {
                Ok(bytes) => buf.extend(bytes),
                Err(StreamError::Closed) => break,
                Err(e) => bail!("failed to read bytes: {e}"),
            }
        }
        buf.shrink_to_fit();
        drop(incoming_req_body_stream);
        IncomingBody::finish(incoming_req_body);
        Ok(buf)
    }
}

struct Routes;

// TODO: refactor this into less of a mess
impl Routes {
    fn products(
        path_rest: &[&str],
        _query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        let method = request.method();

        if !path_rest.is_empty() {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        match method {
            Method::Get => {
                let products = list_products().expect("failed to list products");
                let product_data = products
                    .iter()
                    .map(|product| ProductData::from(product.clone()))
                    .collect::<Vec<ProductData>>();
                let products_json = json!(product_data).to_string();

                response_out.complete_response(200, products_json.as_bytes())
            }
            Method::Post => {
                let body = request.read_body().unwrap();
                let product: Product = serde_json::from_slice::<ProductData>(&body).unwrap().into();
                create_product(&product).expect("failed to create product");
                response_out.complete_response(201, "Created".as_bytes())
            }
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
        }
    }

    fn data_init(
        path_rest: &[&str],
        _query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        let method = request.method();

        if path_rest.len() > 1 {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        match method {
            Method::Get => match path_rest {
                ["all"] => match method {
                    Method::Get => {
                        init_all().expect("failed to initialize products");
                        response_out.complete_response(
                            200,
                            "Products, inventory and orders schema initialized".as_bytes(),
                        )
                    }
                    _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
                },
                ["products"] => match method {
                    Method::Get => {
                        init_products().expect("failed to initialize products");
                        response_out.complete_response(200, "Products initialized".as_bytes())
                    }
                    _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
                },
                ["inventory"] => match method {
                    Method::Get => {
                        init_inventory().expect("failed to initialize inventory");
                        response_out.complete_response(200, "Inventory initialized".as_bytes())
                    }
                    _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
                },
                ["orders"] => match method {
                    Method::Get => {
                        init_orders().expect("failed to initialize orders schema");
                        response_out.complete_response(200, "Orders schema initialized".as_bytes())
                    }
                    _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
                },
                _ => response_out.complete_response(404, b"404 Not Found\n"),
            },
            _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
        }
    }

    fn inventory(
        path_rest: &[&str],
        query: Option<&str>,
        request: IncomingRequest,
        response_out: ResponseOutparam,
    ) {
        if !path_rest.is_empty() {
            return response_out.complete_response(404, b"404 Not Found\n");
        }

        if let None = query {
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
                let mut query_pairs = url::form_urlencoded::parse(query_str.as_bytes());

                let skus_string = query_pairs.find(|(key, _)| key == "skus").unwrap().1;

                let skus_list: Vec<String> =
                    skus_string.split(',').map(|s| s.to_string()).collect();

                let inventory =
                    get_inventory(skus_list.as_slice()).expect("failed to fetch inventory");
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
}

export!(HttpServer);
