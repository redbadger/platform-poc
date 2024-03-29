mod adapters;
mod api;
mod model;

wit_bindgen::generate!({
    world: "products-http-service",
    exports: {
        "wasi:http/incoming-handler": HttpServer,
    },
});

use http::{Request, Response};

use exports::wasi::http::incoming_handler;
use platform_poc::products::products as product_service;
use wasi::http::types as wasi_http;

use api::types::ProductRequest;
use model::Product;

struct HttpServer;

impl incoming_handler::Guest for HttpServer {
    fn handle(request: wasi_http::IncomingRequest, response_out: wasi_http::ResponseOutparam) {
        let request = Request::<Option<ProductRequest>>::try_from(request);

        let response: Response<String> = match request {
            Ok(req) => match req.uri().path() {
                "/api/product" => match req.method().to_owned() {
                    http::Method::GET => {
                        let response = product_service::list_products()
                            .map_err(|e| anyhow::anyhow!(e))
                            .and_then(|r| {
                                let products: Result<Vec<Product>, _> =
                                    r.into_iter().map(|p| p.try_into()).collect();

                                products
                            });

                        match response {
                            Ok(products) => Response::builder()
                                .status(200)
                                .body(serde_json::to_string(&products).unwrap())
                                .expect("failed to build response"),
                            Err(e) => Response::builder()
                                .status(500)
                                .body(format!("Internal server error: {}", e))
                                .expect("failed to build response"),
                        }
                    }
                    http::Method::POST => {
                        if let Some(body) = req.into_body() {
                            let product: Product = body.into();
                            let product: product_service::Product = product.into();

                            match product_service::create_product(&product) {
                                Ok(()) => Response::builder()
                                    .status(201)
                                    .body(String::new())
                                    .expect("failed to build response"),
                                Err(e) => Response::builder()
                                    .status(500)
                                    .body(format!("Internal server error: {}", e))
                                    .expect("failed to build response"),
                            }
                        } else {
                            Response::builder()
                                .status(400)
                                .body("Missing body".to_string())
                                .expect("failed to build response")
                        }
                    }
                    _ => Response::builder()
                        .status(405)
                        .body("Method not allowed".to_string())
                        .expect("failed to build response"),
                },
                _ => Response::builder()
                    .status(404)
                    .body("Not found".to_string())
                    .expect("failed to build response"),
            },
            Err(adapters::http::Error::Serde(e)) => Response::builder()
                .status(400)
                .body(format!("Invalid JSON: {}", e))
                .expect("failed to build response"),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Response::builder()
                    .status(500)
                    .body("Internal server error".to_string())
                    .expect("failed to build response")
            }
        };

        wasi_http::ResponseOutparam::set(response_out, Ok(response.into()));
    }
}
