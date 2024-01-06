mod adapters;
mod api;
mod model;

wit_bindgen::generate!({
    world: "hello",
    exports: {
        "wasi:http/incoming-handler": HttpServer,
    },
});

use api::types::ProductRequest;
use exports::wasi::http::incoming_handler::Guest;
use model::Product;
use wasi::{
    http::types::*,
    keyvalue::{
        readwrite::set,
        types::{new_outgoing_value, open_bucket, outgoing_value_write_body_sync},
    },
};

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let response = OutgoingResponse::new(Fields::new());
        let response_body = response.body().unwrap();

        match http::Request::<Option<ProductRequest>>::try_from(request) {
            Ok(req) => {
                match req.uri().path() {
                    "/api/product" => {
                        match req.method().to_owned() {
                            http::Method::GET => {
                                response_body
                                    .write()
                                    .unwrap()
                                    .blocking_write_and_flush(b"Hello from Rust!\n")
                                    .unwrap();
                                response.set_status_code(200).unwrap();
                            }
                            http::Method::POST => {
                                if let Some(body) = req.into_body() {
                                    let product: Product = body.into();

                                    let outgoing_value = new_outgoing_value();
                                    let bytes =
                                        serde_json::to_vec(&product).expect("failed to serialize");
                                    outgoing_value_write_body_sync(outgoing_value, &bytes)
                                        .expect("failed to write outgoing value");

                                    let bucket =
                                        open_bucket("").expect("failed to open empty bucket");
                                    set(bucket, &product.id.to_string(), outgoing_value)
                                        .expect("failed to set value in bucket");

                                    response_body
                                        .write()
                                        .unwrap()
                                        .blocking_write_and_flush(&bytes)
                                        .unwrap();

                                    response.set_status_code(201).unwrap();
                                } else {
                                    response.set_status_code(400).unwrap();
                                }
                            }
                            _ => {
                                response.set_status_code(405).unwrap();
                            }
                        };
                    }
                    _ => {
                        response.set_status_code(404).unwrap();
                    }
                };
            }
            Err(adapters::http::Error::Serde(e)) => {
                response_body
                    .write()
                    .unwrap()
                    .blocking_write_and_flush(&e.to_string().as_bytes())
                    .unwrap();
                response.set_status_code(400).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                response.set_status_code(500).unwrap();
            }
        }

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        ResponseOutparam::set(response_out, Ok(response));
    }
}
