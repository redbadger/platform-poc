wit_bindgen::generate!({
    world: "http-controller"
});

use anyhow::{anyhow, bail, Result};
use common::products::Product as ProductData;
use exports::wasi::http::incoming_handler::Guest;
use platform_poc::data_init::init_funcs::{init_all, init_inventory, init_orders, init_products};
use platform_poc::products::products::{create_product, list_products, Product};
use serde_json::json;
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

        // skip the first empty string
        let path_parts: Vec<&str> = path.split("/").skip(1).collect();

        match path_parts.as_slice() {
            ["products", path_rest @ ..] => Routes::products(path_rest, request, response_out),
            ["data-init", path_rest @ ..] => Routes::data_init(path_rest, request, response_out),
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
    fn products(path_rest: &[&str], request: IncomingRequest, response_out: ResponseOutparam) {
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

    fn data_init(path_rest: &[&str], request: IncomingRequest, response_out: ResponseOutparam) {
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
}

export!(HttpServer);
