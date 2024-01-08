mod adapters;
mod api;
mod model;

wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:http/incoming-handler": HttpServer,
    },
});

use adapters::keyvalue;
use api::types::ProductRequest;
use exports::wasi::http::incoming_handler::Guest;
use http::{Request, Response};
use model::Product;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(
        request: wasi::http::types::IncomingRequest,
        response_out: wasi::http::types::ResponseOutparam,
    ) {
        let request = Request::<Option<ProductRequest>>::try_from(request);

        let response: Response<String> = match request {
            Ok(req) => match req.uri().path() {
                "/api/product" => match req.method().to_owned() {
                    http::Method::GET => {
                        let products = keyvalue::get_all::<Product>("products").unwrap();

                        Response::builder()
                            .status(200)
                            .body(serde_json::to_string(&products).unwrap())
                            .expect("failed to build response")
                    }
                    http::Method::POST => {
                        if let Some(body) = req.into_body() {
                            let product: Product = body.into();

                            keyvalue::set("", &product.id.to_string(), &product).unwrap();

                            Response::builder()
                                .status(201)
                                .body(serde_json::to_string(&product).unwrap())
                                .expect("failed to build response")
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

        wasi::http::types::ResponseOutparam::set(response_out, Ok(response.into()));
    }
}
