wit_bindgen::generate!({
    world: "http-controller"
});

use anyhow::{anyhow, bail, Result};
use common::products::platform_poc::products::products::{create_product, list_products, Product};
use common::products::Product as ProductData;
use exports::wasi::http::incoming_handler::Guest;
use serde_json::json;
use wasi::http::types::*;
use wasi::io::streams::StreamError;
use wasi::logging::logging::{log, Level};

const MAX_READ_BYTES: u32 = 2048;

struct HttpServer;

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

        // TODO: refactor this into less of a mess
        match path.as_str() {
            "/products" => match method {
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
                    let product: Product =
                        serde_json::from_slice::<ProductData>(&body).unwrap().into();
                    create_product(&product).expect("failed to create product");
                    response_out.complete_response(201, "Created".as_bytes())
                }
                _ => response_out.complete_response(405, b"405 Method Not Allowed\n"),
            },
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

export!(HttpServer);
