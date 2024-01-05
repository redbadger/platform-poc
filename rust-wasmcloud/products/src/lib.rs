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
use wasi::{http::types::*, io::streams::StreamError};

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let response = OutgoingResponse::new(Fields::new());
        let response_body = response.body().unwrap();

        match request.path_with_query().unwrap().as_str() {
            "/api/product" => {
                match request.method() {
                    Method::Get => {
                        response_body
                            .write()
                            .unwrap()
                            .blocking_write_and_flush(b"Hello from Rust!\n")
                            .unwrap();
                        response.set_status_code(200).unwrap();
                    }
                    Method::Post => {
                        let incoming_body = request
                            .consume()
                            .expect("failed to get incoming request body");
                        let stream = incoming_body // don't inline `incoming_body` as it won't consume
                            .stream()
                            .expect("failed to get incoming request stream");
                        let mut buf = vec![];
                        loop {
                            let chunk = match stream.read(1024) {
                                Ok(chunk) => chunk,
                                Err(StreamError::Closed) => break,
                                Err(StreamError::LastOperationFailed(e)) => {
                                    eprintln!("Error reading from stream: {:?}", e);
                                    return;
                                }
                            };
                            buf.extend_from_slice(&chunk);
                        }
                        match serde_json::from_slice::<ProductRequest>(&buf) {
                            Ok(req) => {
                                response_body
                                    .write()
                                    .unwrap()
                                    .blocking_write_and_flush(
                                        serde_json::to_string(&req).unwrap().as_bytes(),
                                    )
                                    .unwrap();

                                response.set_status_code(201).unwrap();
                            }
                            Err(e) => {
                                eprintln!("Error decoding request body: {:?}", e);
                                response_body
                                    .write()
                                    .unwrap()
                                    .blocking_write_and_flush(e.to_string().as_bytes())
                                    .unwrap();

                                response.set_status_code(400).unwrap();
                            }
                        };
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

        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
        ResponseOutparam::set(response_out, Ok(response));
    }
}
